-- Rhyph initial schema — ported from pretix data model
-- All enums stored as TEXT; proper PG enum types added later.

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Organizer (multitenancy root)
CREATE TABLE organizers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    slug VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Events
CREATE TABLE events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organizer_id UUID NOT NULL REFERENCES organizers(id) ON DELETE CASCADE,
    slug VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    location VARCHAR(255) NOT NULL DEFAULT '',
    date_from TIMESTAMPTZ NOT NULL,
    date_to TIMESTAMPTZ,
    timezone VARCHAR(100) NOT NULL DEFAULT 'UTC',
    live BOOLEAN NOT NULL DEFAULT FALSE,
    presale_start TIMESTAMPTZ,
    presale_end TIMESTAMPTZ,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    locale VARCHAR(10) NOT NULL DEFAULT 'en',
    lat DOUBLE PRECISION,
    lon DOUBLE PRECISION,
    max_items_per_order INTEGER NOT NULL DEFAULT 10,
    reservation_time INTEGER NOT NULL DEFAULT 30,
    is_local BOOLEAN NOT NULL DEFAULT TRUE,
    ap_url VARCHAR(512) NOT NULL UNIQUE,
    maximum_attendee_capacity INTEGER,
    remaining_attendee_capacity INTEGER,
    show_remaining_capacity BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(organizer_id, slug)
);

-- Item categories
CREATE TABLE item_categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    position INTEGER NOT NULL DEFAULT 0,
    is_addon BOOLEAN NOT NULL DEFAULT FALSE
);

-- Items (ticket types)
CREATE TABLE items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    category_id UUID REFERENCES item_categories(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    default_price DECIMAL(13,2) NOT NULL DEFAULT 0,
    tax_rate DECIMAL(5,2) NOT NULL DEFAULT 0,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    admission BOOLEAN NOT NULL DEFAULT TRUE,
    personalized BOOLEAN NOT NULL DEFAULT FALSE,
    max_per_order INTEGER,
    min_per_order INTEGER,
    available_from TIMESTAMPTZ,
    available_until TIMESTAMPTZ,
    require_voucher BOOLEAN NOT NULL DEFAULT FALSE,
    hide_without_voucher BOOLEAN NOT NULL DEFAULT FALSE,
    require_approval BOOLEAN NOT NULL DEFAULT FALSE,
    generate_giftcard BOOLEAN NOT NULL DEFAULT FALSE,
    checkin_attention BOOLEAN NOT NULL DEFAULT FALSE,
    validity_mode VARCHAR(20) NOT NULL DEFAULT 'event_default',
    validity_fixed_from TIMESTAMPTZ,
    validity_fixed_until TIMESTAMPTZ,
    validity_dynamic_duration_minutes INTEGER,
    media_policy VARCHAR(20) NOT NULL DEFAULT 'no_policy',
    picture_id UUID,
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Item variations (adult/student/child)
CREATE TABLE item_variations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    item_id UUID NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    value VARCHAR(255) NOT NULL,
    default_price DECIMAL(13,2),
    active BOOLEAN NOT NULL DEFAULT TRUE,
    position INTEGER NOT NULL DEFAULT 0,
    require_approval BOOLEAN NOT NULL DEFAULT FALSE
);

-- Quotas (inventory pools)
CREATE TABLE quotas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    size INTEGER,
    subevent_id UUID,
    close_when_sold_out BOOLEAN NOT NULL DEFAULT FALSE,
    ignore_for_event_availability BOOLEAN NOT NULL DEFAULT FALSE
);

-- Quota ↔ Item join table
CREATE TABLE quota_items (
    quota_id UUID NOT NULL REFERENCES quotas(id) ON DELETE CASCADE,
    item_id UUID NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    PRIMARY KEY (quota_id, item_id)
);

-- Quota ↔ Variation join table
CREATE TABLE quota_variations (
    quota_id UUID NOT NULL REFERENCES quotas(id) ON DELETE CASCADE,
    variation_id UUID NOT NULL REFERENCES item_variations(id) ON DELETE CASCADE,
    PRIMARY KEY (quota_id, variation_id)
);

-- Questions (attendee info collection)
CREATE TABLE questions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    question TEXT NOT NULL,
    type VARCHAR(20) NOT NULL DEFAULT 'text',
    required BOOLEAN NOT NULL DEFAULT FALSE,
    position INTEGER NOT NULL DEFAULT 0,
    items JSONB NOT NULL DEFAULT '[]' -- which items this question applies to
);

CREATE TABLE question_options (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    value VARCHAR(255) NOT NULL,
    position INTEGER NOT NULL DEFAULT 0
);

-- Orders (state machine: pending → paid / expired / canceled)
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE RESTRICT,
    code VARCHAR(16) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    secret VARCHAR(64) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(50),
    locale VARCHAR(10) NOT NULL DEFAULT 'en',
    total DECIMAL(13,2) NOT NULL DEFAULT 0,
    datetime TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires TIMESTAMPTZ,
    payment_provider VARCHAR(50),
    payment_state VARCHAR(20) NOT NULL DEFAULT 'created',
    customer_id UUID,
    testmode BOOLEAN NOT NULL DEFAULT FALSE,
    require_approval BOOLEAN NOT NULL DEFAULT FALSE,
    valid_if_pending BOOLEAN NOT NULL DEFAULT FALSE,
    comment TEXT,
    sales_channel VARCHAR(50) NOT NULL DEFAULT 'web',
    invoice_name VARCHAR(255),
    invoice_company VARCHAR(255),
    invoice_street VARCHAR(255),
    invoice_city VARCHAR(255),
    invoice_zip VARCHAR(20),
    invoice_country VARCHAR(2),
    invoice_vat_id VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(event_id, code)
);

-- Order positions (individual tickets)
CREATE TABLE order_positions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE RESTRICT,
    positionid INTEGER NOT NULL DEFAULT 1,
    item_id UUID NOT NULL REFERENCES items(id) ON DELETE RESTRICT,
    variation_id UUID REFERENCES item_variations(id) ON DELETE SET NULL,
    price DECIMAL(13,2) NOT NULL,
    tax_rate DECIMAL(5,2) NOT NULL DEFAULT 0,
    tax_value DECIMAL(13,2) NOT NULL DEFAULT 0,
    secret VARCHAR(128) NOT NULL,
    attendee_name VARCHAR(255),
    attendee_email VARCHAR(255),
    answers JSONB NOT NULL DEFAULT '[]',
    seat_id UUID,
    pseudonymization_id VARCHAR(128) NOT NULL,
    canceled BOOLEAN NOT NULL DEFAULT FALSE,
    blocked JSONB,
    valid_from TIMESTAMPTZ,
    valid_until TIMESTAMPTZ,
    voucher_budget_use DECIMAL(13,2),
    UNIQUE(order_id, positionid)
);

-- Carts (pre-order reservations)
CREATE TABLE cart_positions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    item_id UUID NOT NULL REFERENCES items(id) ON DELETE RESTRICT,
    variation_id UUID REFERENCES item_variations(id) ON DELETE SET NULL,
    session_key VARCHAR(255) NOT NULL,
    price DECIMAL(13,2) NOT NULL,
    expires TIMESTAMPTZ NOT NULL,
    answers JSONB NOT NULL DEFAULT '[]',
    seat_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cart_positions_session ON cart_positions(session_key);
CREATE INDEX idx_cart_positions_expires ON cart_positions(expires);

-- Seating plans
CREATE TABLE seating_plans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organizer_id UUID NOT NULL REFERENCES organizers(id) ON DELETE CASCADE,
    name VARCHAR(190) NOT NULL,
    layout JSONB NOT NULL
);

-- Seats (individual seat instances per event)
CREATE TABLE seats (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    seating_plan_id UUID NOT NULL REFERENCES seating_plans(id) ON DELETE RESTRICT,
    seat_guid VARCHAR(255) NOT NULL,
    zone_name VARCHAR(255) NOT NULL,
    row_name VARCHAR(255) NOT NULL,
    seat_number VARCHAR(255) NOT NULL,
    x DOUBLE PRECISION NOT NULL DEFAULT 0,
    y DOUBLE PRECISION NOT NULL DEFAULT 0,
    category VARCHAR(100),
    blocked BOOLEAN NOT NULL DEFAULT FALSE,
    order_position_id UUID REFERENCES order_positions(id) ON DELETE SET NULL,
    UNIQUE(event_id, seat_guid)
);

-- Seat category ↔ item mapping
CREATE TABLE seat_category_mappings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    category_name VARCHAR(100) NOT NULL,
    item_id UUID NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    price DECIMAL(13,2) NOT NULL
);

-- Checkin lists (doors/gates)
CREATE TABLE checkin_lists (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    name VARCHAR(190) NOT NULL,
    all_products BOOLEAN NOT NULL DEFAULT TRUE,
    include_pending BOOLEAN NOT NULL DEFAULT FALSE,
    allow_multiple_entries BOOLEAN NOT NULL DEFAULT FALSE,
    allow_entry_after_exit BOOLEAN NOT NULL DEFAULT TRUE,
    rules JSONB NOT NULL DEFAULT '{}'
);

-- Checkin events (scan records)
CREATE TABLE checkins (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    position_id UUID NOT NULL REFERENCES order_positions(id) ON DELETE CASCADE,
    list_id UUID NOT NULL REFERENCES checkin_lists(id) ON DELETE CASCADE,
    datetime TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    type VARCHAR(10) NOT NULL DEFAULT 'entry',
    successful BOOLEAN NOT NULL DEFAULT TRUE,
    error_reason VARCHAR(100),
    device_id UUID,
    gate_id UUID,
    nonce VARCHAR(190),
    forced BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX idx_checkins_position ON checkins(position_id);
CREATE INDEX idx_checkins_list ON checkins(list_id);

-- Vouchers
CREATE TABLE vouchers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    code VARCHAR(255) NOT NULL,
    quota_id UUID REFERENCES quotas(id) ON DELETE SET NULL,
    max_uses INTEGER,
    used INTEGER NOT NULL DEFAULT 0,
    discount_percent DECIMAL(5,2),
    discount_amount DECIMAL(13,2),
    valid_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(event_id, code)
);
