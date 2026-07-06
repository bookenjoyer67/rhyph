<div align="center">

# Rhyph

![Status](https://img.shields.io/badge/status-early_development-ff3e00?style=for-the-badge&labelColor=04060d)
![License](https://img.shields.io/badge/license-AGPLv3-1eff9d?style=for-the-badge&labelColor=04060d)
![Rust](https://img.shields.io/badge/Rust-Axum_0.8-e8c24a?style=for-the-badge&labelColor=04060d)
![SvelteKit](https://img.shields.io/badge/frontend-SvelteKit_5-ff3e00?style=for-the-badge&labelColor=04060d)
![PostgreSQL](https://img.shields.io/badge/database-PostgreSQL-336791?style=for-the-badge&labelColor=04060d)
![Federation](https://img.shields.io/badge/federation-ActivityPub-9d5cff?style=for-the-badge&labelColor=04060d)

**Self-hosted, federated ticketing. Venues and bands run their own instances.**
No third party. No per-ticket fees. No platform rent.

</div>

---

## ✦ What it is

A ticketing platform built for venues, not for ticket brokers. You run your own
instance. You set your own fees. You own your audience data. Federation means a
band playing your venue can sell tickets on their instance and your system
knows about it — or vice versa.

Data model from [pretix](https://pretix.eu). Federation patterns from
[Mobilizon](https://joinmobilizon.org). Built for self-hosters.

**Phase 1 (shipped):** venue ticketing — events, ticket types, quotas, cart,
checkout (Stripe), order management, check-in scanner PWA, admin dashboard,
custom storefront SPAs.

**Phase 2 (planned):** ActivityPub federation — servers discover each other,
events federate, ticket inventory syncs across instances.

---

## ✦ Quick start

```bash
git clone https://github.com/bookenjoyer67/rhyph.git
cd rhyph
cp .env.example .env

# Backend
cargo run --bin rhyph-server

# Frontend (separate terminal)
cd frontend && npm install && npm run dev
```

Open `http://localhost:5173/login` — the setup wizard creates your first admin.

PostgreSQL must be running with a `rhyph` database. Migrations run on startup.

---

## ✦ Architecture

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

---

## ✦ Workspace

```
crates/
├── core/       — Models (16 tables), services, DB migrations, queries
├── api/        — Axum route handlers (events, cart, orders, checkin, auth, admin)
├── federation/ — ActivityPub scaffold (Phase 2)
├── server/     — Binary entry point + SPA config injection middleware
├── payments/   — Stripe checkout + webhook
└── hashpw/     — CLI: Argon2 password hashing
```

---

## ✦ What you can do

- **Sell tickets** — public event pages, cart with session cookies, Stripe checkout
- **Scan at the door** — scanner PWA at `/scan`, manual entry or barcode gun, device registration
- **Run your own storefront** — upload a custom SvelteKit SPA, serve it on your domain
- **Brand it** — per-venue themes: colors, fonts, logos, custom CSS. Five built-in presets
- **Manage** — admin dashboard for events, ticket types, quotas, orders, check-in lists

---

## ✦ Frontend modes

Single SvelteKit 5 SPA, three faces:

| Mode | Path | Auth |
|------|------|------|
| Public storefront | `/`, `/events/*`, `/cart` | None (session cookie) |
| Admin panel | `/admin/*` | JWT (Bearer token) |
| Scanner PWA | `/scan` | API key (`X-API-Key` header) |

All three ship in one `adapter-static` build. No separate apps.

---

## ✦ Custom storefronts

Venues can upload their own SvelteKit build — tar.gz or folder upload. The Rust
backend injects config at serve time (`window.RHYPH = {apiUrl, organizer, theme}`)
so the SPA just works. Max 50MB.

Default storefront is "The Neon Cathedral" — gothic cyberpunk, Cinzel + Space
Grotesk, particle animations. Five preset themes included: Neon Cathedral,
Brutalist, Mojave, Gallery, Festival.

---

## ✦ License

AGPLv3 with custom preamble.
Empire can't fork and close.

