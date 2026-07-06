<script lang="ts">
  import { setToken, getOrganizer } from '$lib/api';
  import { page } from '$app/stores';

  let { children } = $props();

  let token = $state<string | null>(null);

  // Organizer theme state
  let orgTheme = $state<Record<string, unknown> | null>(null);
  let orgName = $state<string | null>(null);

  $effect(() => {
    const stored = localStorage.getItem('rhyph_token');
    if (stored) {
      token = stored;
      setToken(stored);
    }
  });

  // Detect organizer-scoped routes and fetch theme
  $effect(() => {
    const path = $page.url.pathname;
    const params = $page.params;
    const searchParams = $page.url.searchParams;

    let orgSlug: string | null = null;

    // Homepage — load default organizer
    if (path === '/') {
      orgSlug = 'default';
    }
    // /events/{org}/...
    else if (path.startsWith('/events/') && params.org) {
      orgSlug = params.org;
    }
    // /cart?org=X
    else if (path === '/cart' && searchParams.get('org')) {
      orgSlug = searchParams.get('org');
    }
    // /orders/{code}?org=X
    else if (path.startsWith('/orders/') && searchParams.get('org')) {
      orgSlug = searchParams.get('org');
    }

    if (orgSlug) {
      getOrganizer(orgSlug).then(org => {
        orgTheme = org.theme as Record<string, unknown>;
        orgName = org.name;
      }).catch(() => {
        orgTheme = null;
        orgName = null;
      });
    } else {
      orgTheme = null;
      orgName = null;
    }
  });

  function logout() {
    localStorage.removeItem('rhyph_token');
    setToken(null);
    token = null;
  }

  const auth = $derived(!!token);
  const linkStyle = 'color:#aaa;text-decoration:none;padding:8px 16px;border-radius:6px;transition:background .2s;font-size:.9rem';

  // Derive CSS variables from theme
  const primary = $derived(orgTheme?.primary_color as string || '#7c5ce7');
  const accent = $derived(orgTheme?.accent_color as string || '#00E676');
  const logoUrl = $derived(orgTheme?.logo_url as string || null);

  const rootVars = $derived(`--rhyph-primary:${primary};--rhyph-accent:${accent}${orgTheme?.bg_color ? ';--rhyph-bg:' + orgTheme.bg_color : ''}${orgTheme?.card_bg ? ';--rhyph-card-bg:' + orgTheme.card_bg : ''}${orgTheme?.card_border ? ';--rhyph-card-border:' + orgTheme.card_border : ''}${orgTheme?.font_family ? ';--rhyph-font:' + orgTheme.font_family : ''}`);
</script>

<svelte:head>
  {#if orgTheme?.custom_css}
    <style>{orgTheme.custom_css as string}</style>
  {/if}
</svelte:head>

<header style="display:flex;align-items:center;justify-content:space-between;padding:12px 24px;background:var(--rhyph-bg, #0d0d1a);border-bottom:1px solid var(--rhyph-card-border, #2a2a4a)">
  <div style="display:flex;align-items:center;gap:24px">
    {#if orgName && logoUrl}
      <a href="/events/{$page.params.org}" style="display:flex;align-items:center;gap:10px;text-decoration:none">
        <img src={logoUrl} alt={orgName} style="height:32px;width:auto" />
        <span style="font-size:1.25rem;font-weight:700;color:var(--rhyph-primary, #7c5ce7)">{orgName}</span>
      </a>
    {:else if orgName}
      <a href="/events/{$page.params.org}" style="font-size:1.25rem;font-weight:700;color:var(--rhyph-primary, #7c5ce7);text-decoration:none">{orgName}</a>
    {:else}
      <a href="/" style="font-size:1.5rem;font-weight:700;color:var(--rhyph-primary, #7c5ce7);text-decoration:none">Rhyph</a>
    {/if}
  </div>
  <div style="display:flex;align-items:center;gap:12px">
    {#if auth}
      <a href="/admin/events" style={linkStyle}>Events</a>
      <a href="/admin/orders" style={linkStyle}>Orders</a>
      <a href="/admin/checkin" style={linkStyle}>Checkin</a>
      <a href="/admin/devices" style={linkStyle}>Devices</a>
      <a href="/scan" style={linkStyle}>Scanner</a>
      <button
        onclick={logout}
        style="background:var(--rhyph-primary, #7c5ce7);color:#fff;border:none;padding:8px 20px;border-radius:6px;cursor:pointer;font-size:.9rem;margin-left:8px"
        onmouseenter={(e) => (e.currentTarget as HTMLElement).style.opacity = '0.85'}
        onmouseleave={(e) => (e.currentTarget as HTMLElement).style.opacity = '1'}
      >Logout</button>
    {:else}
      <a href="/login" style="background:var(--rhyph-primary, #7c5ce7);color:#fff;text-decoration:none;padding:8px 20px;border-radius:6px;font-size:.9rem;font-weight:600">Login</a>
    {/if}
  </div>
</header>

<main style="min-height:calc(100vh - 61px);background:var(--rhyph-bg, #111);color:#eee;font-family:var(--rhyph-font, system-ui, sans-serif){rootVars}">
  {@render children()}
</main>
