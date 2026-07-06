# Rhyph

Self-hosted, federated ticketing platform. Venues and bands run their own instances.
No third party. No per-ticket fees.

**Status**: Phase 1 complete — fully functional venue ticketing. Federation coming in Phase 2.

## Quick Start

### Option 1: Podman (recommended)

```bash
git clone https://github.com/.../rhyph.git
cd rhyph
cp .env.example .env
npm run build --prefix frontend   # build frontend once
podman compose up -d
```

Open `http://localhost:5173` — login with `admin@rhyph.local` / `admin123`

### Option 2: Bare metal

```bash
git clone https://github.com/.../rhyph.git
cd rhyph
./scripts/setup.sh     # creates DB, builds backend + frontend, seeds data
./scripts/start.sh     # starts the server
```

Open `http://localhost:5173` — login with `admin@rhyph.local` / `admin123`

## What You Can Do

- **Events**: Create events with ticket types, seat maps (coming soon), pricing
- **Sell tickets**: Fans browse public event page → add to cart → checkout
- **Scan at door**: Scanner PWA at `/scan` — manual entry or barcode scanner gun
- **Manage**: Admin dashboard at `/admin/events` — tickets, quotas, checkin lists, devices
- **Payments**: Stripe integration (webhook auto-marks orders paid)

## Architecture

```
  Frontend (SvelteKit)    Backend (Rust/Axum)     PostgreSQL
  ┌──────────────┐       ┌──────────────┐       ┌──────────┐
  │ Admin panel   │──REST─▶│  API routes  │──sqlx─▶│ events    │
  │ Public shop   │       │  Cart/orders │       │ orders    │
  │ Scanner PWA   │       │  Checkin     │       │ tickets   │
  │               │       │  Auth (JWT)  │       │ users     │
  └──────────────┘       └──────────────┘       └──────────┘
```

Built on pretix's data model and Mobilizon's federation patterns. Federation via ActivityPub in Phase 2.

## Development

```bash
# Backend
cargo run --release

# Frontend (separate terminal)
cd frontend && npm install && npm run dev

# Tests (requires test database)
DATABASE_URL=postgres://... cargo test
```

## License

AGPLv3 — empire can't fork and close.
