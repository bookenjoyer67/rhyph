<script lang="ts">
  import { setToken } from '$lib/api';
  import { goto } from '$app/navigation';

  let { children } = $props();

  let token = $state<string | null>(null);
  let checked = $state(false);

  $effect(() => {
    const stored = localStorage.getItem('rhyph_token');
    if (stored) {
      token = stored;
      setToken(stored);
    }
    checked = true;
  });

  $effect(() => {
    if (checked && !token) {
      goto('/login');
    }
  });

  function logout() {
    localStorage.removeItem('rhyph_token');
    setToken(null);
    token = null;
    goto('/');
  }
</script>

{#if !checked || !token}
  <div style="display:flex;align-items:center;justify-content:center;min-height:calc(100vh - 61px);background:#111">
    <p style="color:#aaa;font-size:1.1rem">Redirecting...</p>
  </div>
{:else}
  <nav style="display:flex;align-items:center;justify-content:space-between;padding:12px 24px;background:#0d0d1a;border-bottom:1px solid #2a2a4a">
    <div style="display:flex;align-items:center;gap:24px">
      <a href="/admin/events" style="font-size:1.25rem;font-weight:700;color:#7c5ce7;text-decoration:none">Rhyph Admin</a>
      <a href="/admin/events" style="color:#aaa;text-decoration:none;padding:8px 16px;border-radius:6px;font-size:.9rem">Events</a>
      <a href="/admin/orders" style="color:#aaa;text-decoration:none;padding:8px 16px;border-radius:6px;font-size:.9rem">Orders</a>
      <a href="/admin/checkin" style="color:#aaa;text-decoration:none;padding:8px 16px;border-radius:6px;font-size:.9rem">Checkin</a>
      <a href="/admin/devices" style="color:#aaa;text-decoration:none;padding:8px 16px;border-radius:6px;font-size:.9rem">Devices</a>
    </div>
    <a href="/scan" style="color:#aaa;text-decoration:none;padding:8px 16px;border-radius:6px;font-size:.9rem">Scanner</a>
    <button
      onclick={logout}
      style="background:#7c5ce7;color:#fff;border:none;padding:8px 20px;border-radius:6px;cursor:pointer;font-size:.9rem"
      onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = '#6a4fd4'}
      onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = '#7c5ce7'}
    >Logout</button>
  </nav>

  <main style="min-height:calc(100vh - 61px);background:#111;color:#eee">
    {@render children()}
  </main>
{/if}
