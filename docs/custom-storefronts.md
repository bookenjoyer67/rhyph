# Custom Storefronts — Per-Organizer White-Label Frontends

Status: planning (Phase 2)
Reference: Servhost `customStorefront` pattern + Akkoma frontend management system

## Overview

Each organizer can optionally provide their own static frontend build. When configured,
Rhyph serves that organizer's public routes from their custom build instead of the
default SvelteKit SPA. The API, admin panel, and scanner stay as the standard Rhyph
frontend. The custom storefront becomes the venue's public-facing website.

Fan sees: `venue.com/events/summer-fest` styled like the venue's brand, not Rhyph's.
Admin sees: standard Rhyph admin panel when they log in.
Scanner: unchanged.

## Architecture

```
                         ┌─────────────────────────┐
                         │     Rust/Axum Server     │
                         │                         │
    Request ─────────────▶  Route Dispatcher        │
                         │                         │
    /api/* ──────────────▶  API handlers            │
    /health ─────────────▶  health check            │
    /admin/*, /login     │                         │
    /scan ───────────────▶  default SPA             │──▶ frontend/build/
                         │                         │
    /events/{org}/*      │  ┌─────────────────┐    │
    /cart?org={org}      │  │ organizer has    │    │
    /orders/{code}?org=  │──│ custom_storefront│──▶ storefronts/{org_slug}/
                         │  │ configured?      │    │
                         │  └─────────────────┘    │
                         │        │ NO             │
                         │        └────────────────▶ default SPA
                         │
                         │  Custom domain (host header lookup):
                         │  *.venue.com ──────────▶ same as /events/{org}/* path routing
                         └─────────────────────────┘
```

## Routing Logic (Rust)

```rust
// app.rs — pseudo-layout of the outer Router

Router::new()
    // API is always first
    .nest("/api", api_routes)

    // Health
    .route("/health", get(health))

    // Rhyph-internal SPA routes (never served from custom storefront)
    .route("/admin", ...)
    .route("/admin/*rest", ...)
    .route("/login", ...)
    .route("/scan", ...)

    // Organizer-scoped public routes — dispatch to custom or default
    .fallback(organizer_storefront_dispatcher)
```

### Organizer Storefront Dispatcher

This is the key piece. A Tower `Layer` + `Service` that:

1. Extracts the organizer from the request (path or host header)
2. Looks up `organizers.custom_storefront` and `organizers.custom_domain`
3. If custom storefront configured: serves from `storefronts/{org_slug}/`
4. If not: delegates to the default SPA `ServeDir`

```rust
// crates/server/src/storefront.rs

pub struct OrganizerStorefrontService {
    db: PgPool,
    default_spa: ServeDir,
    config_injector: ConfigInjector, // HTML injection middleware
}

impl Service<Request<Incoming>> for OrganizerStorefrontService {
    async fn call(&self, req: Request<Incoming>) -> Result<Response, Infallible> {
        let organizer = self.resolve_organizer(&req).await;

        match organizer.and_then(|o| o.custom_storefront) {
            Some(path) => {
                // Serve from organizer's custom build
                let dir = ServeDir::new(&path)
                    .fallback(ServeFile::new(format!("{}/index.html", path)));
                // Inject Rhyph config (API URL, org slug) into HTML
                self.config_injector.layer(dir).call(req).await
            }
            None => {
                // Fall back to default SPA
                self.default_spa.clone().call(req).await
            }
        }
    }
}
```

### Organizer Resolution

Two strategies, tried in order:

```rust
fn resolve_organizer(&self, req: &Request) -> Option<Organizer> {
    // 1. Host header → custom_domain match
    if let Some(host) = req.headers().get("host") {
        if let Some(org) = self.db.lookup_by_domain(host) {
            return Some(org);
        }
    }

    // 2. Path prefix → /events/{org_slug}/...
    if let Some(org_slug) = extract_org_from_path(req.uri().path()) {
        return self.db.lookup_by_slug(org_slug);
    }

    None
}
```

## Database Changes

### Migration: `202607XX000001_custom_storefront.sql`

```sql
ALTER TABLE organizers ADD COLUMN custom_storefront VARCHAR(512);
ALTER TABLE organizers ADD COLUMN custom_domain VARCHAR(255);
ALTER TABLE organizers ADD COLUMN theme JSONB NOT NULL DEFAULT '{}';

-- Ensure custom_domain is unique across organizers
CREATE UNIQUE INDEX idx_organizers_custom_domain
    ON organizers(custom_domain) WHERE custom_domain IS NOT NULL;
```

`custom_storefront` — absolute path on the server, e.g. `/var/lib/rhyph/storefronts/fillmore/`
`custom_domain` — e.g. `tickets.thefillmore.com`
`theme` — lightweight fallback when no full custom storefront exists:
```json
{
    "primary_color": "#ff6b35",
    "logo_url": "https://venue.com/logo.png",
    "font_family": "Inter, sans-serif"
}
```

## File System Layout

```
/var/lib/rhyph/
├── storefronts/
│   ├── fillmore/
│   │   ├── index.html
│   │   ├── _app/
│   │   └── events/
│   │       └── summer-fest/
│   │           └── index.html     (SPA fallback handles this too)
│   └── metro/
│       └── ...
└── default/                       (symlink to frontend/build/)
```

The organizer's static build is a standard SvelteKit `adapter-static` output — same
build process as the default SPA, just themed differently.

## Config Injection

Per-organizer config injected into custom storefront HTML via the same
`StorefrontInjectLayer` pattern from Servhost (see `rust-axum-backend` skill,
`references/html-config-injection-middleware.md`).

Injected as `<script>window.RHYPH={...}</script>` before `</head>`:

```json
{
  "apiUrl": "/api/v1",
  "organizer": {
    "slug": "fillmore",
    "name": "The Fillmore",
    "domain": "tickets.thefillmore.com"
  },
  "theme": {
    "primaryColor": "#ff6b35",
    "logoUrl": "https://venue.com/logo.png"
  },
  "features": {
    "customDomain": true,
    "seatingPlans": true
  }
}
```

The custom storefront reads `window.RHYPH` to know which organizer's events to fetch
and how to style itself. No need to hardcode org slugs in the build.

## Organizer Experience

### Setting up a custom storefront

1. Organizer builds their SvelteKit frontend (or hires a dev) — same API surface,
   their branding
2. In Rhyph admin panel → Organizer Settings → "Custom Storefront":
   - Upload a `.tar.gz` of their static build
   - Or paste a path on the server (for self-hosted instances)
   - Set a custom domain (optional)
   - Preview before activating
3. Rhyph extracts the archive to `storefronts/{org_slug}/`
4. Toggle "Use Custom Storefront" on

### Default theming (no custom build)

For organizers who don't want a full custom build, the `theme` JSONB column drives
CSS custom properties on the default SPA:

```css
:root {
  --rhyph-primary: #7c5ce7;       /* default */
  --rhyph-logo: url('/logo.svg');
}

/* When organizer theme is loaded: */
:root {
  --rhyph-primary: #ff6b35;       /* from organizer.theme.primary_color */
  --rhyph-logo: url('https://venue.com/logo.png');
}
```

The default SPA reads the organizer theme from the API (or from `window.RHYPH`
when injected at serve-time) and applies CSS variables.

## Implementation Phases

### Phase 2a — Database + Theme Column (minimal)

- Migration: `custom_storefront`, `custom_domain`, `theme` on `organizers`
- Admin API: PATCH `/api/v1/admin/organizers/{slug}` to set theme
- Default SPA: reads organizer theme on `/events/{org}/*` routes, applies CSS vars
- No custom storefront serving yet — just the theme column

This gives venues immediate branding control without a custom build.

### Phase 2b — Custom Storefront Serving (Servhost pattern)

- Rust server serves static files (add `ServeDir` + `tower-http/fs` feature)
- `OrganizerStorefrontService` dispatcher
- Config injection middleware
- Admin API: upload/storefront management
- Admin UI: organizer settings page with upload widget

### Phase 2c — Custom Domains

- Host header resolution in `OrganizerStorefrontService`
- Admin UI: domain field with validation
- SSL/docs for pointing CNAME

## Tradeoffs & Edge Cases

**Path-based vs domain-based routing:**
Path-based (`/events/{org}/*`) works immediately with zero DNS config. Domain-based
(`tickets.venue.com`) gives full white-label but requires DNS setup. We do both:
path always works, domain is optional.

**Cart and order pages:**
Current routes use query params (`/cart?org=X&event=Y`), not path params. A custom
storefront would handle `/cart` and `/orders/{code}` itself — reading org from the
injected config or query params. The dispatcher routes these to the custom storefront
when the request's origin matches that organizer (via referrer, cookie, or explicit
query param).

**SPA fallback in custom storefronts:**
Custom storefronts are SPAs too — `/events/{org}/{slug}` resolves to `index.html`
via `ServeDir::fallback(ServeFile)`. Same pattern as the default SPA.

**API URL in custom storefronts:**
The injected `window.RHYPH.apiUrl` is always `/api/v1` (same-origin). Custom domains
work transparently — the API is served from the same domain. No CORS issues.

**Security:**
Organizers can only upload static assets (HTML/CSS/JS/images). No server-side code.
Upload size limits enforced. File type validation on extraction. Custom storefront
served from a dedicated directory, never overlaps with the default SPA or admin routes.

## Crate Structure Changes

```
crates/
├── server/
│   ├── src/
│   │   ├── main.rs
│   │   ├── app.rs
│   │   ├── storefront.rs          ← NEW: OrganizerStorefrontService
│   │   └── config_inject.rs       ← NEW: StorefrontInjectLayer (from Servhost)
```

New dependencies:
```toml
# tower-http already included, just add the "fs" feature
tower-http = { features = ["cors", "trace", "limit", "fs"] }

# For body manipulation in injection middleware
http-body = "1"
http-body-util = "0.1"
bytes = "1"
```

## Default SPA Changes

The default SPA already has the public routes (`/events/{org}/{slug}`, `/cart`,
`/orders/{code}`). Changes needed:

1. **Read organizer theme**: On `/events/{org}/*` and `/cart?org=X`, fetch
   organizer config and apply CSS variables
2. **Config injection aware**: Check for `window.RHYPH` and use it if present
3. **Branded header**: Public layout shows organizer logo/name when on
   organizer-scoped routes, falls back to "Rhyph" on generic pages

These changes to the default SPA are **Phase 2a** — they work immediately after
the `theme` column exists, no Rust changes required.
