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

### Phase 2a — Theme Column + Custom CSS (minimal, no Rust changes)

- Migration: `theme` JSONB + `custom_domain` on `organizers`
- Admin API: PATCH `/api/v1/admin/organizers/{slug}` to set theme
- Default SPA: reads organizer theme on `/events/{org}/*` routes, applies CSS vars
- Custom CSS injection: `theme.custom_css` field → injected via `<style>` tag
- Zero Rust changes. One migration, small SvelteKit changes.

This gives venues immediate branding control: `{ "primary_color": "#ff6b35", "logo_url": "https://...", "custom_css": ".event-card { border-radius: 0; }" }`

### Phase 2b — Frontend Registry (Akkoma marketplace pattern)

- `frontends` table (name, version, source_url, source_type, local_path)
- Admin API: `POST /frontends/install`, `GET /frontends`, `DELETE /frontends/{id}`
- `organizer.frontend_id` FK → pick a frontend
- `OrganizerStorefrontService` dispatcher in Rust (Servhost pattern)
- Config injection middleware (`window.RHYPH` → HTML)
- Admin UI: theme browser + one-click apply
- Ship 2-3 community themes as proof of concept

Organizer flow: admin → themes → browse "Mojave" / "Neon" / "Brutalist" → preview → apply. No build, no upload, no dev.

### Phase 2c — Custom Upload + Custom Domains

- Organizer upload: tar.gz → extracted to `storefronts/{org_slug}/`
- `frontends.source_type = 'custom'` entries
- Host header resolution in dispatcher
- Admin UI: domain field, upload widget
- Splash screen injection from theme JSON
- SSL/docs for CNAME setup

### Phase 2d (stretch) — Fan-Level Override

- Fan can switch to default Rhyph view even when organizer uses custom storefront
- Stored in cookie/localStorage
- Same pattern as Akkoma's per-user frontend picker

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
These changes to the default SPA are **Phase 2a** — they work immediately after
the `theme` column exists, no Rust changes required.

## Akkoma-Inspired Patterns

Akkoma's frontend management system is the most mature example in fediverse
software. Key patterns to borrow:

### 1. Frontend Registry ("Theme Marketplace")

Akkoma has `frontends.available` in config — a catalog of known frontends with
names, GitLab repos, and install commands. Rhyph gets a `frontends` table:

```sql
CREATE TABLE frontends (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL UNIQUE,       -- "mojave", "neon", "brutalist"
    display_name VARCHAR(255) NOT NULL,       -- "Mojave Desert Theme"
    description TEXT,
    version VARCHAR(50) NOT NULL,             -- "1.2.0"
    source_url VARCHAR(512),                  -- GitHub release download URL
    source_type VARCHAR(20) NOT NULL DEFAULT 'community',
        -- 'system'   = shipped with Rhyph (the default SPA)
        -- 'community' = listed in the theme marketplace
        -- 'custom'    = organizer uploaded
    local_path VARCHAR(512),                  -- extracted to storefronts/themes/{name}/{version}/
    author VARCHAR(255),
    thumbnail_url VARCHAR(512),
    installed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Which frontend an organizer uses
ALTER TABLE organizers ADD COLUMN frontend_id UUID REFERENCES frontends(id);
```

This turns "upload your own static build" into a browsable catalog. An organizer
goes to admin → themes → picks "Mojave" → previews → clicks "Apply." Zero
technical skill needed.

### 2. Frontend Install via Admin API (not CLI)

Akkoma uses `mix pleroma.frontend install <name> --ref <ref>`. Rhyph does it
via admin API instead — no shell access needed:

```
POST /api/v1/admin/frontends/install
{
    "name": "mojave",
    "source_url": "https://github.com/rhyph-themes/mojave/releases/download/v1.2.0/dist.tar.gz"
}
→ Server downloads, extracts to storefronts/themes/mojave/v1.2.0/
→ Registers in frontends table
→ Returns { id, name, version, preview_url }
```

```
GET /api/v1/admin/frontends
→ [{ id, name: "mojave", display_name: "Mojave Desert", version: "1.2.0",
     source_type: "community", thumbnail_url: "..." }, ...]

POST /api/v1/admin/organizers/{slug}/frontend
{ "frontend_id": "uuid-of-mojave" }
→ Sets organizer.frontend_id → custom storefront is live
```

```
DELETE /api/v1/admin/frontends/{id}
→ Removes from disk and registry (unless an organizer is using it)
```

### 3. Three Levels of Customization (lightest → heaviest)

Inspired by Akkoma's approach of having both lightweight config overrides AND
full frontend replacement:

| Level | What it is | Tech skill needed | Akkoma analog |
|-------|------------|-------------------|---------------|
| **Theme JSON** | `organizer.theme` column — colors, logo, font | Zero | `custom.css` + PleromaFE admin config |
| **Installed theme** | Pick from marketplace, one-click apply | Zero | `frontends.available` → pick in settings |
| **Custom upload** | Build your own SPA, upload tar.gz | Developer | Manual frontend install from ZIP |

Level 1 (theme JSON) applies to the default SPA via CSS variables — no separate
build needed. Level 2 is the frontend registry. Level 3 is the Servhost pattern
(full custom build).

### 4. Splash Screen / Pre-Loader

Akkoma's PleromaFE customization guide documents a branded splash screen — logo
+ name + tagline on black background, shown before the SPA mounts. Rhyph can do
the same for custom storefronts:

- Organizer sets `theme.splash_logo_url` and `theme.splash_tagline`
- Config injector adds a `<div id="rhyph-splash">` to index.html at serve time
- Splash removes itself when the SPA mounts (MutationObserver on app root)

This makes the venue's brand visible INSTANTLY, even before JavaScript loads.
Critical for venues on slow connections (festivals, rural areas).

### 5. Per-Organizer Frontend Selection

Akkoma lets users pick their preferred frontend in Settings → Frontends.
Rhyph lets organizers pick their storefront. Same pattern, different actor.

The `organizer.frontend_id` column is the selection. The dispatcher reads it
at request time. An organizer can switch themes without affecting their events,
tickets, or orders — just the paint layer changes.

**Stretch goal**: Fan-level frontend override. A fan browsing `fillmore.events`
could switch to the default Rhyph view if they prefer it. Stored in a cookie
or localStorage. Same pattern as Akkoma's per-user frontend picker.

### 6. Admin Panel Is Always Standard

Akkoma's `admin-fe` is a SEPARATE installable frontend, never themed by user
preferences. Rhyph formalizes this:

- `/admin/*`, `/login`, `/scan` → always served from the default SPA
- The admin `+layout.svelte` and scanner `+layout@.svelte` are NEVER overridden
- Only `/events/{org}/*` and organizer-scoped public routes get dispatched

This is already true in the current architecture, but Akkoma's explicit
separation of `admin-fe` as its own package makes it a deliberate design
choice rather than an accident of routing.

### 7. Static Assets Pitfall (Learned from Akkoma)

When Akkoma sets a custom frontend as `primary`, the SPA catch-all (`/*path`
fallback) intercepts asset requests — the browser loads `index.html`, requests
`/assets/main.js`, and receives `index.html` back instead. Blank page.

**Rhyph avoids this by design:**
- Custom storefronts are served from `/events/{org}/*` paths only, not root
- Asset paths in custom builds are relative (the SvelteKit default)
- `ServeDir` correctly resolves `/events/fillmore/_app/immutable/chunks/...`
  to the custom storefront's file tree
- No catch-all at root that would swallow asset requests

**But verify during implementation:** test with a real custom build that asset
paths resolve correctly under the path-prefixed routing.

### 8. Custom CSS Injection (Akkoma `custom.css` Pattern)

Akkoma loads `instance/static/static/custom.css` on every page without a
rebuild. Rhyph can do the same:

- Organizer sets `theme.custom_css` (text field in admin)
- Config injector wraps it in `<style id="rhyph-custom">` and injects it
- Organizer can tweak colors, hide elements, add custom fonts — no build needed

This is the frictionless entry point. A venue owner pastes 20 lines of CSS and
their event page changes color. No developer, no build, no upload.

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
