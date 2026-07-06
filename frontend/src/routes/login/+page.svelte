<script lang="ts">
  import { login, setup, setToken } from '$lib/api';
  import { goto } from '$app/navigation';

  let { data } = $props();

  let email = $state('');
  let password = $state('');
  let confirmPassword = $state('');
  let error = $state('');
  let loading = $state(false);

  const isSetup = $derived(data.needsSetup ?? false);

  // Redirect if already logged in
  $effect(() => {
    const token = localStorage.getItem('rhyph_token');
    if (token) {
      setToken(token);
      goto('/admin/events');
    }
  });

  async function handleSetup(e: Event) {
    e.preventDefault();
    loading = true;
    error = '';
    try {
      if (password !== confirmPassword) {
        error = 'Passwords do not match';
        loading = false;
        return;
      }
      const res = await setup(email, password);
      localStorage.setItem('rhyph_token', res.token);
      setToken(res.token);
      goto('/admin/events');
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Setup failed';
    } finally {
      loading = false;
    }
  }

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

  const onSubmit = $derived(isSetup ? handleSetup : handleLogin);
  const title = $derived(isSetup ? 'Setup' : 'Login');
  const subtitle = $derived(isSetup ? 'Create your admin account' : '');
  const buttonLabel = $derived(isSetup ? (loading ? 'Creating...' : 'Create Admin Account') : (loading ? 'Logging in...' : 'Login'));
</script>

<div style="display:flex;align-items:center;justify-content:center;min-height:calc(100vh - 61px);background:#111;padding:24px">
  <form
    onsubmit={onSubmit}
    style="display:flex;flex-direction:column;gap:16px;width:100%;max-width:400px;padding:40px;background:#1a1a2e;border-radius:12px;border:1px solid #2a2a4a"
  >
    <h1 style="font-size:1.8rem;font-weight:700;color:#7c5ce7;text-align:center;margin:0">{title}</h1>

    {#if isSetup}
      <p style="color:#aaa;text-align:center;font-size:.9rem;margin:0">
        This is a fresh Rhyph instance. Create the first admin account to get started.
      </p>
    {/if}

    {#if subtitle}
      <p style="color:#4ade80;text-align:center;margin:0;font-size:.9rem">{subtitle}</p>
    {/if}

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
        type="password" bind:value={password} required minlength="8"
        style="padding:12px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none"
      />
    </label>

    {#if isSetup}
      <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">
        Confirm Password
        <input
          type="password" bind:value={confirmPassword} required minlength="8"
          style="padding:12px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none"
        />
      </label>
    {/if}

    <button
      type="submit" disabled={loading}
      style="background:#7c5ce7;color:#fff;border:none;padding:12px;border-radius:6px;cursor:pointer;font-size:1rem;font-weight:600;margin-top:8px;opacity:{loading ? 0.6 : 1}"
    >
      {buttonLabel}
    </button>

    {#if !isSetup}
      <a href="/" style="color:#aaa;text-align:center;font-size:.85rem;text-decoration:none;margin-top:4px">&larr; Back</a>
    {/if}
  </form>
</div>
