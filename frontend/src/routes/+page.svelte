<script lang="ts">
  import { login, setToken } from '$lib/api';
  import { goto } from '$app/navigation';

  let email = $state('');
  let password = $state('');
  let error = $state('');
  let loading = $state(false);

  $effect(() => {
    const token = localStorage.getItem('rhyph_token');
    if (token) {
      setToken(token);
      goto('/admin/events');
    }
  });

  async function handleLogin(e: Event) {
    e.preventDefault();
    loading = true;
    error = '';
    try {
      const res = await login(email, password);
      localStorage.setItem('rhyph_token', res.token);
      setToken(res.token);
      goto('/admin/events');
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Login failed';
    } finally {
      loading = false;
    }
  }
</script>

<div style="display:flex;align-items:center;justify-content:center;min-height:100vh;background:#111">
  <form
    onsubmit={handleLogin}
    style="display:flex;flex-direction:column;gap:16px;width:100%;max-width:400px;padding:40px;background:#1a1a2e;border-radius:12px;border:1px solid #2a2a4a"
  >
    <h1 style="font-size:1.8rem;font-weight:700;color:#7c5ce7;text-align:center;margin:0">Rhyph Admin</h1>

    {#if error}
      <p style="color:#e74c3c;margin:0;text-align:center;font-size:.9rem">{error}</p>
    {/if}

    <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">
      Email
      <input
        type="email" bind:value={email} required
        style="padding:12px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none"
      />
    </label>

    <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">
      Password
      <input
        type="password" bind:value={password} required
        style="padding:12px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none"
      />
    </label>

    <button
      type="submit" disabled={loading}
      style="background:#7c5ce7;color:#fff;border:none;padding:12px;border-radius:6px;cursor:pointer;font-size:1rem;font-weight:600;margin-top:8px;opacity:{loading ? 0.6 : 1}"
      onmouseenter={(e) => { if (!loading) (e.currentTarget as HTMLElement).style.background = '#6a4fd4' }}
      onmouseleave={(e) => { if (!loading) (e.currentTarget as HTMLElement).style.background = '#7c5ce7' }}
    >
      {loading ? 'Logging in...' : 'Login'}
    </button>
  </form>
</div>
