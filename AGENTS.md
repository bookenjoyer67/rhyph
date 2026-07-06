# Rhyph — Developer Reference

Venue-first federated ticketing platform. AGPLv3.

## Quick Start

```bash
cp .env.example .env
# Backend
cargo run --bin rhyph-server
# Frontend
cd frontend && npm install && npm run dev
```

PostgreSQL must be running with a `rhyph` database. Migrations run on startup.

## Architecture

```
  Frontend (SvelteKit 5)     Backend (Rust/Axum 0.8)     PostgreSQL
  ┌──────────────┐          ┌──────────────┐            ┌──────────┐
  │ Admin panel   │──REST──▶│  API routes   │───sqlx 0.8─▶│ 16 tables │
  │ Public shop   │         │  Cart/orders  │            │ pretix    │
  │ Scanner PWA   │         │  Checkin      │            │ data model│
  │ Custom SPA    │         │  Auth (JWT)   │            └──────────┘
  └──────────────┘          │  Stripe       │
       :5173                │  Federation   │
   (Vite dev)               └──────────────┘
                                  :3000
```

Frontend serves separately from backend — the Rust binary is a JSON API only.
Vite proxies `/api` and `/health` to `:3000`.

## Workspace (6 crates)

```
crates/
├── core/       — Models (16 tables), services, DB migrations, queries
├── api/        — Axum route handlers (events, cart, orders, checkin, auth, admin) + middleware
├── federation/ — ActivityPub scaffold (Phase 2)
├── server/     — Binary entry point (main.rs + app.rs + spa_config.rs)
├── payments/   — Stripe integration (checkout + webhook)
└── hashpw/     — CLI: Argon2 password hashing
```

## Data Model

pretix-derived, 16 tables. Uses `bigdecimal` for money (NOT `rust_decimal`).

organizers → events → items/item_variations/quotas → orders/order_positions/cart_positions → checkin_lists/checkins. Plus users, devices, seating_plans/seats, vouchers, questions, organizer_images.

## Custom SPA System

Organizers can upload custom SvelteKit `adapter-static` builds served from `public_pages/{org_slug}/spa/`. The `SpaConfigLayer` (Tower middleware) injects `window.RHYPH = {apiUrl, organizer, theme}` before `</head>` in HTML responses. Two upload modes: folder upload (multipart with `webkitRelativePath`) and tar.gz upload. Max 50MB.

Default SPA at `public_pages/default/spa/` — served as the catch-all fallback.

## Frontend Architecture

Single SvelteKit 5 SPA with `adapter-static` (SPA fallback). Three "modes" via layout chain:
- **Public storefront** — root `+layout.svelte` (public, minimal header)
- **Admin panel** — `admin/+layout.svelte` (JWT-gated, redirects to `/login`)
- **Scanner PWA** — `scan/+layout@.svelte` (layout reset, API-key-gated)

Cart uses session cookies (`cart_session`, HttpOnly, 30min). Scanner auth uses `X-API-Key` header. All API routes are public — only device registration requires `RequireAdmin`.

## Auth

- Admin: JWT (Bearer token, `Authorization` header), Argon2 password hashing
- Setup wizard: if `users` table is empty, `/login` becomes a setup form. `GET /api/v1/auth/needs-setup` → `POST /api/v1/auth/setup {email, password}`. Returns 409 once a user exists.

## Pitfalls

### ServeDir paths must be absolute
`ServeDir::new("public_pages/default/spa")` resolves relative to the process cwd — breaks across `cargo run`, systemd, and shell. Use `env!("CARGO_MANIFEST_DIR")` with `.parent().unwrap().parent().unwrap()` to resolve the project root at compile time.

### SpaConfigLayer Content-Length mismatch
Injecting `window.RHYPH` changes body size but `ServeDir` attaches the original `Content-Length` header. Must call `response.headers_mut().remove(http::header::CONTENT_LENGTH)` after injection or hyper panics.

### bigdecimal vs rust_decimal
Data model uses `bigdecimal` (not `rust_decimal`). rust_decimal is a workspace dep but NOT the active one for models.

### Svelte 5: $state captures initial value only
Never use `$state(props.someValue)` for props/load data. `$state` snapshots SSR value and never updates on client hydration. Use `const x = $derived(data.someValue)` for anything from `$props()` or SvelteKit `data`.

### Svelte 5: $effect with async .then() unreliable for initial data
Use `+page.ts` load function with `export const ssr = false` instead. Load runs before render, data passes via `$derived()`.

### Setup wizard DB state
Testing the setup wizard leaves the DB dirty — an admin user exists, wizard is disabled. Wipe DB (`DROP DATABASE rhyph; CREATE DATABASE rhyph;`) after testing so the user sees the wizard on next visit.

### Federation crate scaffolded, not wired
`crates/federation/` exists with ActivityPub types but `routes/federation.rs` is empty. Phase 2.

### adapter-static SPA fallback
All routes serve `index.html` — the Rust backend or nginx must handle API routes first.

### .svelte-kit/ must be gitignored
Build artifacts. `.gitignore` covers it now — if you see `.svelte-kit/` files in git status, they're stale tracked files that need `git rm --cached`.

## Build

```bash
cargo build --bin rhyph-server     # backend
cd frontend && npm run build       # frontend (outputs to frontend/build/)
cd custom-spas/neon-cathedral && npm install && npm run build  # custom SPA
```

## License

AGPLv3 with custom preamble. See `LICENSE`.
