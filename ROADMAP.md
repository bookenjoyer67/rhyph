# Development Roadmap — Rhyph

Status as of July 6, 2026. Checked = done, blank = not started.

This is early-stage software. It runs. You can create events, sell tickets,
and scan people in. It is not production-ready. Federation is scaffolding.
Payments are functional but need hardening.

---

## Phase 1: Core Ticketing ✓

- [x] Events CRUD (create, list, get, update)
- [x] Ticket types (items, item variations, categories)
- [x] Quota management (available counts, per-item limits)
- [x] Cart with session cookies (add, remove, view, expire)
- [x] Order creation (pending → paid → checked-in lifecycle)
- [x] Check-in (manual code entry, device registration, API key auth)
- [x] Admin dashboard (events, orders, check-in, devices)
- [x] Scanner PWA (`/scan` — manual entry, barcode scanner gun support)
- [x] Auth — JWT, Argon2 passwords, web setup wizard
- [x] DB migrations — full pretix-derived schema (16 tables)

## Phase 2: Venue Customization

- [x] Organizer themes — CSS custom properties, 5 built-in presets
- [x] Organizer images — upload, list, delete (logo, hero, etc.)
- [x] Custom SPA upload — tar.gz or folder, served with config injection
- [x] Default storefront — "The Neon Cathedral" gothic cyberpunk SPA
- [ ] Custom domains — Host header resolution for white-label venues

## Phase 3: Payments (in progress)

- [x] Stripe checkout session creation
- [x] Stripe webhook verification (HMAC-SHA256)
- [x] Payment status → order status (paid → confirmed)
- [ ] Refund handling
- [ ] Payment failure recovery UX
- [ ] Success/cancel URL flow end-to-end
- [ ] Test mode vs live mode toggle
- [ ] Multi-currency support
- [ ] Alternative payment methods (cash-at-door, invoice)

## Phase 4: Seating & Capacity

- [x] DB schema for seating plans, seats, category mappings
- [ ] API — create/edit seat maps
- [ ] Seat selection in cart
- [ ] Reserved seating vs general admission

## Phase 5: Federation

- [x] ActivityPub crate scaffolded (types, converter, actions, publisher)
- [ ] WebFinger endpoint
- [ ] Actor profiles (venues, events as actors)
- [ ] Event federation (Create/Update Activity)
- [ ] Ticket inventory sync across instances
- [ ] Cross-instance order flow
- [ ] Federation routes wired into the API

## Phase 6: Advanced Ticketing

- [ ] Vouchers / discount codes (model exists, no API)
- [ ] Custom questions per ticket (model exists, no API)
- [ ] Gift cards (model exists, no API)
- [ ] Waitlists
- [ ] Timed entry slots
- [ ] Recurring events

---

## Production Readiness

- [ ] Test suite (zero tests right now)
- [ ] CI/CD pipeline
- [ ] Docker Compose verified for fresh clone
- [ ] Production deployment guide (nginx, systemd, TLS)
- [ ] Rate limiting tuned for real traffic
- [ ] Database backup/restore documented
- [ ] Migration rollback strategy
- [ ] Error monitoring (Sentry or equivalent)
- [ ] Admin onboarding documentation

---

## Known Gaps

**Payments.** The Stripe provider creates checkout sessions and verifies
webhooks. What's missing: the frontend checkout flow end-to-end (cart →
Stripe redirect → success page → order confirmation), refund handling,
and production hardening. This works in isolation but hasn't been
battle-tested.

**Federation.** The `crates/federation/` crate has real types (actors,
activities, converter, publisher) but the `routes/federation.rs` file is
one line — a comment. No federation endpoint is wired into the router.
Phase 2 item.

**Tests.** None. Zero. The entire codebase is untested. DB queries,
auth flow, cart logic, order state machine — all hand-verified but not
automated.

**Security.** No security audit has been performed. JWT secret rotation,
rate limiting defaults, CORS policy, SQL injection surface — all
unreviewed.
