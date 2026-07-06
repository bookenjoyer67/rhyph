<script lang="ts">
  import { setToken } from '$lib/api';

  let { children } = $props();

  let token = $state<string | null>(null);

  $effect(() => {
    const stored = localStorage.getItem('rhyph_token');
    if (stored) {
      token = stored;
      setToken(stored);
    }
  });

  function logout() {
    localStorage.removeItem('rhyph_token');
    setToken(null);
    token = null;
  }

  const auth = $derived(!!token);
  const linkStyle = 'color:#aaa;text-decoration:none;padding:8px 16px;border-radius:6px;transition:background .2s;font-size:.9rem';
</script>

<header style="display:flex;align-items:center;justify-content:space-between;padding:12px 24px;background:#0d0d1a;border-bottom:1px solid #2a2a4a">
  <div style="display:flex;align-items:center;gap:24px">
    <a href="/" style="font-size:1.5rem;font-weight:700;color:#7c5ce7;text-decoration:none">Rhyph</a>
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
        style="background:#7c5ce7;color:#fff;border:none;padding:8px 20px;border-radius:6px;cursor:pointer;font-size:.9rem;margin-left:8px"
        onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = '#6a4fd4'}
        onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = '#7c5ce7'}
      >Logout</button>
    {:else}
      <a href="/login" style="background:#7c5ce7;color:#fff;text-decoration:none;padding:8px 20px;border-radius:6px;font-size:.9rem;font-weight:600">Login</a>
    {/if}
  </div>
</header>

<main style="min-height:calc(100vh - 61px);background:#111;color:#eee">
  {@render children()}
</main>
