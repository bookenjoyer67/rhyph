# Rhyph — Self-Hosted Federated Ticketing

A venue-first, self-hosted ticketing platform with ActivityPub federation.
Venues and bands run their own instances. No third party. No per-ticket fees.

**Data model**: pretix (Python/Django) — ported to Rust
**Federation**: Mobilizon (Elixir/Phoenix) — ActivityPub patterns, ported to Rust

## Table of Contents

1. [Overview](#overview)
2. [Architecture Diagram](#architecture-diagram)
3. [Data Model](#data-model)
4. [Federation Protocol](#federation-protocol)
5. [API Surface](#api-surface)
6. [Crate Structure](#crate-structure)
7. [Implementation Phases](#implementation-phases)
8. [Tech Decisions](#tech-decisions)

---

## Overview

Rhyph is two products sharing one codebase and one federation protocol:

| Product | Who runs it | What it does |
|---------|-------------|--------------|
| **Venue instance** | Venues, clubs, theaters | Full ticketing: seat maps, payments, QR codes, door scanning, box office |
| **Band instance** | Touring artists | Lightweight: tour page, presales, fan emails. Same binary, fewer modules. |

Both instances federate via ActivityPub. A band's tour dates appear on venue instances. A fan's tickets aggregate across instances. Event discovery spans the network without a central directory.

**What Rhyph replaces**: Ticketmaster's exclusive venue contracts, per-ticket service fees, and closed resale marketplace. A venue runs Rhyph on their own hardware (or a VPS) and owns the entire relationship — customer data, payment flow, door access.

**What Rhyph does NOT replace**: The secondary market (resale). Not in scope for v1. A ticket lives on the instance that sold it.

---

## Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────┐
│                     FEDERATION LAYER                              │
│                  (ActivityPub + HTTP Signatures)                   │
│                                                                    │
│   ┌─────────────────────┐          ┌─────────────────────┐        │
│   │   VENUE INSTANCE     │          │   BAND INSTANCE      │        │
│   │   (full ticketing)   │          │   (tour management)  │        │
│   │                      │          │                      │        │
│   │  ┌────────────────┐  │          │  ┌────────────────┐  │        │
│   │  │  Admin Panel   │  │          │  │  Tour Page     │  │        │
│   │  │  (SvelteKit)   │  │          │  │  (SvelteKit)   │  │        │
│   │  └───────┬────────┘  │          │  └───────┬────────┘  │        │
│   │  ┌───────┴────────┐  │          │  ┌───────┴────────┐  │        │
│   │  │  Public Shop   │  │          │  │  Fan Emails    │  │        │
│   │  │  (SvelteKit)   │  │          │  │  Presale Codes │  │        │
│   │  └───────┬────────┘  │          │  └───────┬────────┘  │        │
│   │  ┌───────┴────────┐  │          │  ┌───────┴────────┐  │        │
│   │  │  Scanner PWA   │  │          │  │  Analytics     │  │        │
│   │  │  (door scan)   │  │          │  │  Dashboard     │  │        │
│   │  └───────┬────────┘  │          │  └───────┬────────┘  │        │
│   │          │           │          │          │           │        │
│   │  ┌───────┴───────────────────────────────┴────────┐   │        │
│   │  │              REST API (Axum 0.8)                │   │        │
│   │  │  ┌──────────┐ ┌──────────┐ ┌───────────────┐  │   │        │
│   │  │  │  Events   │ │  Orders  │ │  Checkin      │  │   │        │
│   │  │  │  Items    │ │  Cart    │ │  Federation   │  │   │        │
│   │  │  │  Quotas   │ │  Tickets │ │  Inbox/Outbox │  │   │        │
│   │  │  │  Seating  │ │  Payment │ │  WebFinger    │  │   │        │
│   │  │  └──────────┘ └──────────┘ └───────────────┘  │   │        │
│   │  └──────────────────────┬────────────────────────┘   │        │
│   │                         │                            │        │
│   │                 ┌───────┴───────┐                    │        │
│   │                 │  PostgreSQL   │                    │        │
│   │                 │  (sqlx)       │                    │        │
│   │                 └───────────────┘                    │        │
│   └─────────────────────┘          └─────────────────────┘        │
│                                                                    │
│   Federation objects shared between instances:                     │
│   • Event (AP standard — venue, time, description, image)          │
│   • TicketOffer (price, currency, purchase URL, capacity)          │
│   • Venue (location, capacity, contact)                            │
│   • Tour (collection of Events)                                    │
└──────────────────────────────────────────────────────────────────┘
```

---

## Data Model

The data model is a Rust port of pretix's battle-tested PostgreSQL schema.
Every model is a SQLx query struct. No ORM. Explicit queries.

### Organizer (multitenancy root)

```rust
// One organizer can manage many events across many venues.
// This is the billing/ownership boundary.
struct Organizer {
    id: Uuid,
    slug: String,         // used in URLs: /{organizer_slug}/events/{event_slug}/
    name: String,
    // settings: JSON blob for per-organizer config
}
```

### Event

```rust
struct Event {
    id: Uuid,
    organizer_id: Uuid,
    slug: String,
    name: String,
    description: String,
    location: String,
    date_from: DateTime<Utc>,
    date_to: Option<DateTime<Utc>>,
    timezone: String,              // "America/Chicago"
    live: bool,                    // publicly visible
    presale_start: Option<DateTime<Utc>>,
    presale_end: Option<DateTime<Utc>>,
    currency: String,              // "USD"
    locale: String,                // "en"
    // Geo for maps
    lat: Option<f64>,
    lon: Option<f64>,
    // Settings
    max_items_per_order: i32,
    reservation_time: i32,         // minutes before cart expires
    // Federation
    is_local: bool,                // created on this instance
    ap_url: String,                // ActivityPub ID: https://venue.com/events/{uuid}
    // Capacity tracking (mirrors Mobilizon's EventOptions)
    maximum_attendee_capacity: Option<i32>,
    remaining_attendee_capacity: Option<i32>,
    show_remaining_capacity: bool,
}
```

### Item (ticket type)

```rust
// "General Admission", "VIP", "Early Bird", "Student"
struct Item {
    id: Uuid,
    event_id: Uuid,
    category_id: Option<Uuid>,     // optional grouping (GA, VIP, Add-ons)
    name: String,                  // "General Admission"
    description: Option<String>,
    default_price: Decimal,        // 25.00
    tax_rate: Decimal,             // 8.5 (percent)
    active: bool,                  // currently on sale
    admission: bool,               // grants entry (vs. merchandise, donation)
    personalized: bool,            // requires attendee name/info
    max_per_order: Option<i32>,    // max quantity per order
    min_per_order: Option<i32>,
    available_from: Option<DateTime<Utc>>,
    available_until: Option<DateTime<Utc>>,
    require_voucher: bool,         // only purchasable with voucher code
    hide_without_voucher: bool,    // hidden until voucher entered
    require_approval: bool,        // order needs staff approval
    generate_giftcard: bool,       // purchasing this creates a gift card
    checkin_attention: bool,       // flag for door staff
    // Ticket validity
    validity_mode: ValidityMode,   // EventDefault | Fixed | Dynamic
    validity_fixed_from: Option<DateTime<Utc>>,
    validity_fixed_until: Option<DateTime<Utc>>,
    validity_dynamic_duration_minutes: Option<i32>,
    // Reusable media (RFID/NFC wristbands)
    media_policy: MediaPolicy,
    // Display
    picture_id: Option<Uuid>,
    position: i32,                 // sort order in shop
}
```

### ItemVariation

```rust
// Same ticket, different price points: "Adult", "Student", "Child"
struct ItemVariation {
    id: Uuid,
    item_id: Uuid,
    value: String,                 // "Student"
    default_price: Option<Decimal>, // overrides item price if set
    active: bool,
    position: i32,
    require_approval: bool,
}
```

### Quota (inventory pool — THE critical model)

```rust
// A quota is a pool of capacity that items draw from.
// Multiple items can share a quota. One item can be in multiple quotas.
// Availability = min(available in each quota that applies).
//
// Example:
//   Quota "Total Capacity" { size: 500, items: [GA, VIP] }
//   Quota "VIP Limit"      { size:  50, items: [VIP] }
//   → VIP availability = min(500_total_remaining, 50_vip_remaining)

struct Quota {
    id: Uuid,
    event_id: Uuid,
    name: String,                  // "Total Capacity"
    size: Option<i32>,             // None = unlimited
    items: Vec<Uuid>,              // which Items draw from this pool (M2M)
    variations: Vec<Uuid>,         // which Variations (M2M)
    subevent_id: Option<Uuid>,     // for event series (multi-date)
    close_when_sold_out: bool,
    ignore_for_event_availability: bool,
}
```

### Order (state machine)

```rust
// STATUS: Pending → Paid | Expired | Canceled
// Pending orders reserve quota. Expired/canceled release it.
struct Order {
    id: Uuid,
    event_id: Uuid,
    code: String,                  // short human-readable: "F8K2L"
    status: OrderStatus,           // Pending | Paid | Expired | Canceled
    secret: String,                // secret URL token for customer access
    email: Option<String>,
    phone: Option<String>,
    locale: String,                // "en"
    total: Decimal,
    datetime: DateTime<Utc>,
    expires: Option<DateTime<Utc>>, // when pending reservation expires
    payment_provider: String,       // "stripe", "paypal", "manual"
    payment_state: PaymentState,   // Created | Pending | Confirmed | Failed | Refunded
    customer_id: Option<Uuid>,     // if logged into venue instance
    testmode: bool,
    require_approval: bool,
    valid_if_pending: bool,        // treat as valid even if unpaid
    comment: Option<String>,       // internal staff note
    sales_channel: String,         // "web", "box_office", "reseller"
    // Invoice address
    invoice_name: Option<String>,
    invoice_company: Option<String>,
    invoice_street: Option<String>,
    invoice_city: Option<String>,
    invoice_zip: Option<String>,
    invoice_country: Option<String>,
    invoice_vat_id: Option<String>,
    // Timestamps
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

### OrderPosition (individual ticket)

```rust
// One OrderPosition = one scannable ticket.
// Each has a unique secret used for QR code generation.
struct OrderPosition {
    id: Uuid,
    order_id: Uuid,
    positionid: i32,               // 1,2,3... within order
    item_id: Uuid,
    variation_id: Option<Uuid>,
    price: Decimal,
    tax_rate: Decimal,
    tax_value: Decimal,
    secret: String,                // QR code content (signed token)
    attendee_name: Option<String>,
    attendee_email: Option<String>,
    // Custom attendee fields (question answers stored as JSON)
    answers: serde_json::Value,    // [{"question_id": X, "answer": "XL"}]
    seat_id: Option<Uuid>,         // null for GA tickets
    pseudonymization_id: String,   // for lead scanning (different from secret)
    canceled: bool,
    blocked: Option<Vec<String>>,  // reasons blocked from entry (null = not blocked)
    valid_from: Option<DateTime<Utc>>,
    valid_until: Option<DateTime<Utc>>,
    voucher_budget_use: Option<Decimal>,
}
```

### SeatingPlan + Seat

```rust
// Venue creates a seating plan: zones → rows → seats.
// Each seat has a globally unique GUID.
// Events map seat categories to ticket types/prices.
struct SeatingPlan {
    id: Uuid,
    organizer_id: Uuid,
    name: String,                  // "Main Hall"
    layout: serde_json::Value,     // JSON schema:
    // {
    //   "zones": [{
    //     "name": "Orchestra",
    //     "position": {"x": 100, "y": 200},
    //     "rows": [{
    //       "row_number": "A",
    //       "seats": [{
    //         "seat_guid": "abc123",
    //         "seat_number": "12"
    //       }]
    //     }]
    //   }],
    //   "categories": [{"name": "VIP"}, {"name": "Standard"}]
    // }
}

struct Seat {
    id: Uuid,
    event_id: Uuid,
    seating_plan_id: Uuid,
    seat_guid: String,             // from the layout JSON
    zone_name: String,
    row_name: String,
    seat_number: String,
    x: f64, y: f64,               // position for clickable map
    category: Option<String>,      // "VIP", "Standard"
    blocked: bool,                 // manually blocked (obstructed view, broken)
    order_position_id: Option<Uuid>, // null = available
}

struct SeatCategoryMapping {
    id: Uuid,
    event_id: Uuid,
    category_name: String,         // "VIP" (from seating plan)
    item_id: Uuid,                 // which ticket type maps to this category
    price: Decimal,                // can override item default
}
```

### CheckinList + Checkin

```rust
// A checkin list is a door/gate configuration.
// Multiple lists = multiple entrances (Main, VIP, Backstage, etc.)
struct CheckinList {
    id: Uuid,
    event_id: Uuid,
    name: String,                  // "Main Entrance"
    all_products: bool,            // scan any ticket type
    limit_products: Vec<Uuid>,     // or restrict to specific items
    include_pending: bool,         // allow unpaid if valid_if_pending
    allow_multiple_entries: bool,  // don't warn on re-scan
    allow_entry_after_exit: bool,  // re-entry allowed
    rules: serde_json::Value,      // JSON Logic for custom rules
    // e.g. {"and": [
    //   {">=": [{"var": "entries_today"}, 1]},
    //   {"==": [{"var": "product"}, "VIP"]}
    // ]}
}

struct Checkin {
    id: Uuid,
    position_id: Uuid,             // which ticket
    list_id: Uuid,               // which entrance
    datetime: DateTime<Utc>,
    type_: CheckinType,            // Entry | Exit
    successful: bool,
    error_reason: Option<String>,  // if !successful — Canceled | Unpaid | AlreadyRedeemed | ...
    device_id: Option<Uuid>,       // which scanner device
    gate_id: Option<Uuid>,
    nonce: Option<String>,         // prevent duplicate scans
    forced: bool,                  // offline scan, would've failed online
}
```

### Supporting Models

```rust
// ItemCategory — groups items in the shop ("GA", "VIP", "Add-ons")
// Voucher — discount codes, pre-sale access, gift vouchers
// GiftCard — prepaid balance
// Question — attendee info collection ("T-shirt size?", "Dietary restrictions?")
// QuestionOption — choices for dropdown/radio questions
// Invoice — generated PDF invoices (v2)
// CartPosition — session-based pre-order cart items
// Device — registered scanner devices with API keys
// Gate — physical gate for auto-configuring devices

// Federation-specific:
// Activity — outbound AP activities awaiting delivery
// RemoteActor — cached remote actor data
// Tombstone — deleted objects for federation
```

---

## Federation Protocol

Rhyph uses ActivityPub for server-to-server federation. The reference implementation
is Mobilizon's federation layer (`lib/federation/`), ported to Rust using the
`activitypub_federation` crate from LemmyNet.

### Actor Model

```
┌──────────────────────────────────────────┐
│  Actor Types                              │
│                                           │
│  Venue  —  tickets.thefillmore.com       │
│           manages events, sells tickets   │
│                                           │
│  Band   —  tickets.bandname.com          │
│           publishes tour dates, presales  │
│                                           │
│  Person —  alice@venue-a.com             │
│           buys tickets, receives updates  │
│                                           │
│  Group  —  (future) fan club, street team│
└──────────────────────────────────────────┘
```

### ActivityPub Objects

#### Event (AP standard, extended)

```json
{
  "type": "Event",
  "id": "https://venue.com/events/abc123",
  "attributedTo": "https://venue.com/actor",
  "name": "Band Name — Spring Tour 2026",
  "content": "<p>Doors at 7pm, opener at 8pm</p>",
  "startTime": "2026-03-15T20:00:00-05:00",
  "endTime": "2026-03-15T23:00:00-05:00",
  "location": {
    "type": "Place",
    "name": "The Fillmore",
    "address": {
      "streetAddress": "1805 Geary Blvd",
      "addressLocality": "San Francisco",
      "addressRegion": "CA",
      "postalCode": "94115"
    }
  },
  "maximumAttendeeCapacity": 500,
  "remainingAttendeeCapacity": 237,
  "category": "music",
  "inLanguage": "en",
  "timezone": "America/Chicago",

  // Rhyph extension: ticket offers
  "offers": [
    {
      "type": "Offer",
      "name": "General Admission",
      "price": 25.00,
      "priceCurrency": "USD",
      "url": "https://venue.com/events/abc123/ga",
      "availability": "InStock"
    },
    {
      "type": "Offer",
      "name": "VIP",
      "price": 75.00,
      "priceCurrency": "USD",
      "url": "https://venue.com/events/abc123/vip",
      "availability": "LimitedAvailability"
    }
  ],

  // Rhyph extension: venue capabilities
  "venueCapabilities": {
    "seatingPlan": true,
    "doorScanning": true,
    "boxOffice": true
  }
}
```

#### Ticket (Rhyph extension — new AP object type)

```json
{
  "type": "Ticket",
  "id": "https://venue.com/tickets/order123/pos1",
  "attributedTo": "https://venue.com/actor",
  "event": "https://venue.com/events/abc123",
  "owner": "https://venue-a.com/users/alice",
  "seat": "Orchestra-RowA-Seat12",
  "status": "valid",
  "validFrom": "2026-03-15T19:00:00-05:00",
  "price": 25.00,
  "priceCurrency": "USD"
}
```

The `Ticket` object ONLY contains non-sensitive metadata. The QR code secret
and validation state are NEVER federated. They live exclusively on the selling
instance's database. When a fan views a remote ticket in their "My Tickets"
aggregation, they're redirected to the selling instance for QR display and
validation.

### Federation Flows

#### Flow 1: Event Discovery

```
  Band instance                          Venue instance
  ─────────────                          ──────────────
  1. Band creates "Spring Tour 2026"
     event at The Fillmore
  2. Event published with
     attributedTo = band
     to = [band.followers, venue.followers]
  3. AP Create activity
     POSTs to venue inbox  ──────────→  4. Receives Create activity
                                        5. Transmogrifier processes Event
                                        6. Creates local Event copy
                                        7. Links ticket purchase URL
                                           to band's instance
```

#### Flow 2: Cross-Instance Purchase (v2)

```
  Fan @ venue-a.com                     venue-b.com (selling)
  ──────────────                        ──────────────
  1. Fan views event on venue-b
     (discovered via federation)
  2. Clicks "Buy Tickets"
  3. WebFinger lookup:
     "alice@venue-a.com"  ──────────→  4. Returns actor URL
  5. venue-a signs identity
     confirmation            ──────────→  6. venue-b creates guest
                                           account for alice
                                        7. Purchase completes
                                        8. Ticket lives on venue-b
  9. Receives Ticket
     activity (metadata)  ←──────────  10. AP Ticket object
                                           sent to venue-a
  11. Appears in "My Tickets"
      on venue-a.com
```

#### Flow 3: Capacity Sync (v2+)

```
  Band instance                          Venue instances (A, B, C)
  ─────────────                          ──────────────
  1. Band allocates:
     Venue A: 200 tickets
     Venue B: 150 tickets
     Direct:  50 tickets
  2. Sends TicketAllocation
     activities                ──────────→  Each venue receives
                                           their allocation
  3. Venues sell from
     their allocation
  4. Sold-out notification
     when allocation depleted  ←──────────  Venue A sells out
```

### HTTP Signatures

All server-to-server requests are signed using HTTP Signatures (RFC 9421).
Each actor has an Ed25519 key pair. The `activitypub_federation` crate handles
signing and verification.

### WebFinger

Actor discovery: `alice@venue-a.com` → GET `https://venue-a.com/.well-known/webfinger?resource=acct:alice@venue-a.com` → returns ActivityPub actor URL and inbox/outbox endpoints.

### Inbox/Outbox Pattern

```
POST /actor/inbox   — receive activities from remote instances
GET  /actor/outbox  — list activities published by this actor
GET  /actor/followers — paginated followers collection
```

The federation layer is an I/O adapter. Core business logic (quotas, payments,
checkins) never calls federation directly. Instead, the service layer emits
events, and a background worker translates them to AP activities and delivers
them. This is exactly how Mobilizon's `Federator` GenServer works.

---

## API Surface

REST API (not GraphQL — simpler for MVP). All endpoints under `/api/v1/`.

### Events

```
GET    /api/v1/organizers/{org}/events              — list events
POST   /api/v1/organizers/{org}/events              — create event
GET    /api/v1/organizers/{org}/events/{slug}       — get event
PATCH  /api/v1/organizers/{org}/events/{slug}       — update event
DELETE /api/v1/organizers/{org}/events/{slug}       — delete event

GET    /api/v1/organizers/{org}/events/{slug}/items  — list ticket types
POST   /api/v1/organizers/{org}/events/{slug}/items  — create ticket type
```

### Orders

```
POST   /api/v1/organizers/{org}/events/{slug}/cart/     — add to cart
GET    /api/v1/organizers/{org}/events/{slug}/cart/     — view cart
DELETE /api/v1/organizers/{org}/events/{slug}/cart/{id} — remove from cart

POST   /api/v1/organizers/{org}/events/{slug}/orders/   — create order from cart
GET    /api/v1/organizers/{org}/events/{slug}/orders/{code}/ — get order
POST   /api/v1/organizers/{org}/events/{slug}/orders/{code}/pay/ — initiate payment
GET    /api/v1/organizers/{org}/events/{slug}/orders/{code}/tickets/ — download tickets
```

### Checkin (API key auth — scanner devices)

```
GET    /api/v1/checkin/lists/{id}/status/              — get checkin stats
POST   /api/v1/checkin/lists/{id}/scan/                — scan a ticket
       Body: {"secret": "abc123...", "type": "entry"}
       Response: {"success": true, "position": {...}} | {"success": false, "reason": "already_redeemed"}

POST   /api/v1/checkin/lists/{id}/bulk_scan/           — upload offline scans
```

### Federation

```
POST   /api/v1/federation/inbox                        — receive AP activities
GET    /api/v1/federation/actor/{username}             — actor profile
GET    /api/v1/federation/actor/{username}/outbox      — published activities
GET    /api/v1/federation/actor/{username}/followers   — followers collection
GET    /.well-known/webfinger?resource=acct:{user}@{domain}  — WebFinger
GET    /.well-known/nodeinfo                           — instance metadata
```

### Admin

```
POST   /api/v1/admin/events/{slug}/quotas/             — manage quotas
GET    /api/v1/admin/events/{slug}/orders/             — list all orders
POST   /api/v1/admin/events/{slug}/orders/{code}/refund/ — refund order
POST   /api/v1/admin/events/{slug}/seating-plan/       — upload seat layout
GET    /api/v1/admin/events/{slug}/checkin/stats/      — checkin statistics
POST   /api/v1/admin/devices/                          — register scanner device
```

---

## Crate Structure

```
rhyph/
├── Cargo.toml                  # workspace
├── crates/
│   ├── core/                   # Data models + business logic (pretix port)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models/         # SQLx query structs
│   │       │   ├── mod.rs
│   │       │   ├── organizer.rs
│   │       │   ├── event.rs
│   │       │   ├── item.rs     # Item, ItemVariation, ItemCategory
│   │       │   ├── quota.rs    # Quota + availability logic
│   │       │   ├── order.rs    # Order, OrderPosition, CartPosition
│   │       │   ├── checkin.rs  # CheckinList, Checkin
│   │       │   ├── seating.rs  # SeatingPlan, Seat, SeatCategoryMapping
│   │       │   ├── voucher.rs
│   │       │   ├── question.rs
│   │       │   └── federation.rs # Activity, RemoteActor, Tombstone
│   │       ├── services/       # Business logic
│   │       │   ├── mod.rs
│   │       │   ├── cart.rs     # Add/remove items, quota validation
│   │       │   ├── orders.rs   # Cart→Order, payment, cancellation
│   │       │   ├── quotas.rs   # Availability calculation with locking
│   │       │   ├── checkin.rs  # Validate scan, record entry/exit
│   │       │   ├── tickets.rs  # Secret generation, QR, PDF
│   │       │   ├── seating.rs  # Seat assignment
│   │       │   ├── pricing.rs  # Price calc with tax, discounts
│   │       │   └── payments.rs # Payment provider interface
│   │       └── db/
│   │           ├── mod.rs
│   │           ├── migrations/ # SQLx migrations (pretix schema → Rust)
│   │           └── queries/    # Typed SQL queries
│   │
│   ├── api/                    # Axum REST API
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── routes/
│   │       │   ├── mod.rs
│   │       │   ├── events.rs
│   │       │   ├── orders.rs
│   │       │   ├── cart.rs
│   │       │   ├── checkin.rs
│   │       │   ├── admin.rs
│   │       │   └── federation.rs  # Inbox, outbox, WebFinger
│   │       ├── middleware/
│   │       │   ├── auth.rs        # JWT + API key
│   │       │   ├── organizer.rs   # Org scope resolution
│   │       │   └── rate_limit.rs
│   │       └── error.rs           # Unified error responses
│   │
│   ├── federation/             # ActivityPub integration
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── activity_pub.rs    # Main AP context (fetch/process objects)
│   │       ├── converter/         # Bidirectional: internal ↔ AS2
│   │       │   ├── mod.rs
│   │       │   ├── event.rs       # Event → AS2 Event (+ TicketOffer)
│   │       │   ├── actor.rs       # Actor → AS2 Person/Group
│   │       │   └── ticket.rs      # Ticket → AS2 Ticket (v2)
│   │       ├── transmogrifier.rs  # INBOUND: AS2 JSON → internal model
│   │       ├── publisher.rs       # OUTBOUND: internal → AS2 → HTTP POST
│   │       ├── audience.rs        # Calculate to/cc/addressing
│   │       └── actions/           # AP activity handlers
│   │           ├── mod.rs
│   │           ├── create.rs      # Handle Create activities
│   │           ├── update.rs      # Handle Update activities
│   │           ├── delete.rs      # Handle Delete activities
│   │           ├── follow.rs      # Handle Follow + Accept
│   │           └── announce.rs    # Handle Announce (shares)
│   │
│   ├── server/                  # Binary entry point
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── config.rs        # Env/config loading
│   │       └── app.rs           # Axum router assembly
│   │
│   └── payments/                # Payment provider implementations
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs           # PaymentProvider trait
│           ├── stripe.rs
│           └── manual.rs        # Cash/check (box office)
│
├── frontend/                    # SvelteKit SPA
│   ├── package.json
│   ├── src/
│   │   ├── routes/
│   │   │   ├── +page.svelte          # Venue public shop
│   │   │   ├── events/[slug]/
│   │   │   │   ├── +page.svelte      # Event detail + ticket selection
│   │   │   │   └── checkout/
│   │   │   │       └── +page.svelte  # Checkout flow
│   │   │   ├── orders/[code]/
│   │   │   │   └── +page.svelte      # Order confirmation + ticket download
│   │   │   ├── admin/                # Admin panel (auth-gated)
│   │   │   │   ├── +layout.svelte
│   │   │   │   ├── events/
│   │   │   │   ├── seating/
│   │   │   │   ├── checkin/
│   │   │   │   └── orders/
│   │   │   └── scan/                 # Scanner PWA (camera access)
│   │   │       └── +page.svelte
│   │   └── lib/
│   │       ├── api.ts                # API client
│   │       └── stores/               # Svelte stores
│   └── static/
│
├── scanner/                     # PWA for door scanning
│   └── (may be part of frontend/ or separate)
│
├── scripts/
│   ├── dev.sh                   # cargo watch + svelte dev
│   └── deploy.sh                # build + scp to server
│
├── ARCHITECTURE.md              # This file
├── README.md
├── LICENSE                      # AGPLv3
└── AGENTS.md                    # Developer reference
```

---

## Implementation Phases

### Phase 1 — Core Ticketing (venue instance, no federation)

**Goal**: One venue sells tickets, takes payment, generates QR codes, scans at door.

- [ ] Database schema: all core models (event, item, quota, order, position, checkin, seat)
- [ ] Services: cart, orders (create from cart), quotas (availability with locking), tickets (secret generation)
- [ ] API: event CRUD, cart CRUD, order create/get, checkin scan endpoint
- [ ] Payment: Stripe integration (single provider)
- [ ] Scanner PWA: camera-based QR scanning, entry/exit, offline queue
- [ ] Frontend: public shop (event page → add to cart → checkout → order confirmation)
- [ ] Admin panel: event setup, ticket type creation, quota management, order list
- [ ] Seat map editor: SVG-based, save/load layout JSON
- [ ] Deployment: single binary + PostgreSQL, Docker Compose

**Verification**: Buy a ticket, scan it, verify checkin recorded.

### Phase 2 — Federation (event discovery)

**Goal**: Band instance publishes events; venue instances receive them. Cross-instance event visibility.

- [ ] Federation crate: ActivityPub integration via `activitypub_federation`
- [ ] Actor model: venue/band/person actors with key pairs
- [ ] Event federation: AS2 Event creation, outbound delivery, inbound Transmogrifier
- [ ] WebFinger endpoint
- [ ] HTTP Signatures (signing + verification)
- [ ] Inbox/outbox endpoints
- [ ] Band instance mode: lightweight deployment (tour page, presale codes, fan email)
- [ ] Event discovery: venue instances show federated events with purchase links

**Verification**: Band creates event → appears on venue instance. Venue sells tickets (link-out to band instance).

### Phase 3 — Identity Bridge

**Goal**: Fan uses one identity across instances. Aggregated "My Tickets" view.

- [ ] Cross-instance WebFinger authentication
- [ ] Guest account creation on remote instance at purchase time
- [ ] Ticket metadata federation (Ticket object, no secrets)
- [ ] "My Tickets" aggregation across instances
- [ ] Email/presale capture for bands

**Verification**: Alice@venue-a buys ticket on venue-b using venue-a identity. Ticket appears in venue-a "My Tickets".

### Phase 4 — Allocation + Capacity Sync

**Goal**: Band allocates ticket blocks to venues. Real-time capacity updates.

- [ ] TicketAllocation AP object
- [ ] Allocation management UI (band instance)
- [ ] Capacity sync between instances (periodic or event-driven)
- [ ] Sold-out notification

### Phase 5 — Polish

- [ ] Multiple payment providers (PayPal, bank transfer, cash)
- [ ] Voucher/gift card system
- [ ] PDF ticket generation
- [ ] Apple/Google Wallet passes
- [ ] Reporting + analytics dashboard
- [ ] Multi-language
- [ ] Accessible UI (WCAG 2.1 AA)
- [ ] Instance discovery/registry (optional, not a central directory)

---

## Tech Decisions

| Decision | Choice | Why |
|----------|--------|-----|
| **Language** | Rust | Performance, correctness, your stack |
| **Web framework** | Axum 0.8 | Async, type-safe, your stack |
| **Database** | PostgreSQL | Transactions for quota locking, seat assignments |
| **DB driver** | SQLx | Compile-time query checking, no ORM |
| **Frontend** | SvelteKit (SPA mode) | Your stack, deploy to Cloudflare Pages |
| **Scanner app** | PWA (SvelteKit, camera access) | No app store, offline capable, installable |
| **Federation** | ActivityPub (activitypub_federation crate) | W3C standard, existing fediverse ecosystem |
| **Auth (API)** | JWT (access + refresh) + API keys (scanner devices) | Standard, stateless |
| **Auth (Fed)** | HTTP Signatures + Ed25519 | Required by ActivityPub spec |
| **Payments** | Stripe (v1), plugin trait for extensibility | Don't build payment processing from scratch |
| **Background jobs** | Tokio tasks + PostgreSQL LISTEN/NOTIFY | No external queue dependency for MVP |
| **Deployment** | Single binary + PostgreSQL + CF Pages (frontend) | Simple, no Kubernetes needed |
| **License** | AGPLv3 | Empire can't fork and close |

### What we DON'T use

| Avoid | Why |
|-------|-----|
| ORM (Diesel/SeaORM) | pretix's Django ORM created impedance mismatch. SQLx gives exact control over quota-locking queries |
| GraphQL | pretix uses REST. Mobilizon uses GraphQL but it adds complexity. REST is simpler for MVP |
| Blockchain/NFTs | Solves a problem that doesn't exist. QR codes + server-side validation are enough |
| Microservices | One binary, one database. Don't distribute until you have to |
| Redis | PostgreSQL is enough for MVP caching. Add Redis when quota availability becomes a bottleneck |
| Central directory/registry | Intercommunalist principle: no bootstrap dependency. Discovery through federation + existing web |

### Key design invariants

1. **Ticket secrets NEVER leave the selling instance.** QR codes are validated at the door by the instance that sold them. Federation is metadata-only.
2. **Quota locking is atomic.** The cart→order transition must be correct under concurrent load. PostgreSQL row-level locking, not application-level.
3. **Federation is async and best-effort.** A ticket purchase completes immediately. Federation delivery is background. No two-phase commit across instances.
4. **Offline-first scanning.** The scanner PWA caches valid ticket secrets. If the venue network dies, scanning continues and syncs when connectivity returns.
5. **One binary, multiple modes.** `--mode venue` starts the full stack. `--mode band` starts a lightweight deployment with venue modules disabled. Same codebase, same federation protocol.

---

## References

- **pretix** — https://github.com/pretix/pretix (data model, quota system, order lifecycle)
- **Mobilizon** — https://framagit.org/kaihuri/mobilizon (federation architecture, Transmogrifier pattern, Actor model)
- **activitypub_federation** — https://github.com/LemmyNet/activitypub-federation-rust (Rust AP crate, HTTP Signatures, WebFinger)
- **ActivityPub spec** — https://www.w3.org/TR/activitypub/
- **ActivityStreams 2.0** — https://www.w3.org/TR/activitystreams-core/
