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

  function handleInitialAuth(e: Event) {
    e.preventDefault();
    const form = e.currentTarget as HTMLFormElement;
    const data = new FormData(form);
    const email = data.get('email') as string;
    const password = data.get('password') as string;
    import('$lib/api').then(({ login }) => {
      login(email, password).then((res: { token: string }) => {
        localStorage.setItem('rhyph_token', res.token);
        setToken(res.token);
        token = res.token;
      }).catch(() => {});
    });
  }

  function logout() {
    localStorage.removeItem('rhyph_token');
    setToken(null);
    token = null;
  }

  const auth = $derived(!!token);
  const navLink = (path: string) => `color:#aaa;text-decoration:none;padding:8px 16px;border-radius:6px;transition:background .2s`;
  const navActive = (path: string) => {
    if (typeof window !== 'undefined' && window.location.pathname.startsWith(path)) {
      return 'background:#333;color:#fff';
    }
    return '';
  };
</script>

{#if auth}
  <nav style="display:flex;align-items:center;justify-content:space-between;padding:12px 24px;background:#0d0d1a;border-bottom:1px solid #2a2a4a">
    <div style="display:flex;align-items:center;gap:24px">
      <a href="/" style="font-size:1.5rem;font-weight:700;color:#7c5ce7;text-decoration:none">Rhyph</a>
      <a href="/admin/events" style={navLink('/admin/events') + navActive('/admin/events')}>Events</a>
      <a href="/admin/orders" style={navLink('/admin/orders') + navActive('/admin/orders')}>Orders</a>
      <a href="/admin/checkin" style={navLink('/admin/checkin') + navActive('/admin/checkin')}>Checkin</a>
      <a href="/admin/devices" style={navLink('/admin/devices') + navActive('/admin/devices')}>Devices</a>
    </div>
    <button
      onclick={logout}
      style="background:#7c5ce7;color:#fff;border:none;padding:8px 20px;border-radius:6px;cursor:pointer;font-size:.9rem"
      onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = '#6a4fd4'}
      onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = '#7c5ce7'}
    >
      Logout
    </button>
  </nav>
{/if}

<main style="min-height:calc(100vh - 61px);background:#111;color:#eee">
  {#if auth}
    {@render children()}
  {:else}
    <div style="display:flex;align-items:center;justify-content:center;min-height:100vh;background:#111">
      <form
        onsubmit={handleInitialAuth}
        style="display:flex;flex-direction:column;gap:16px;width:100%;max-width:400px;padding:40px;background:#1a1a2e;border-radius:12px;border:1px solid #2a2a4a"
      >
        <h1 style="font-size:1.8rem;font-weight:700;color:#7c5ce7;text-align:center;margin:0">Rhyph Admin</h1>
        <input
          type="email" name="email" placeholder="Email" required
          style="padding:12px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none"
        />
        <input
          type="password" name="password" placeholder="Password" required
          style="padding:12px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none"
        />
        <button
          type="submit"
          style="background:#7c5ce7;color:#fff;border:none;padding:12px;border-radius:6px;cursor:pointer;font-size:1rem;font-weight:600;margin-top:8px"
          onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = '#6a4fd4'}
          onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = '#7c5ce7'}
        >
          Login
        </button>
      </form>
    </div>
  {/if}
</main>
