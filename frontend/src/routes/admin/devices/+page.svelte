<script lang="ts">
  import { listDevices, createDevice } from '$lib/api';
  import { goto } from '$app/navigation';

  interface Device {
    id: string;
    name: string;
    api_key: string;
    created_at: string;
  }

  let devices = $state<Device[]>([]);
  let loading = $state(true);
  let error = $state('');
  let showForm = $state(false);
  let formLoading = $state(false);
  let formError = $state('');
  let deviceName = $state('');
  let newApiKey = $state('');
  let copied = $state(false);

  $effect(() => {
    if (!localStorage.getItem('rhyph_token')) {
      goto('/');
      return;
    }
    loadDevices();
  });

  async function loadDevices() {
    loading = true;
    error = '';
    try {
      devices = await listDevices();
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Failed to load devices';
    } finally {
      loading = false;
    }
  }

  async function handleCreate(e: Event) {
    e.preventDefault();
    if (!deviceName) return;
    formLoading = true;
    formError = '';
    try {
      const res = await createDevice(deviceName);
      newApiKey = res.api_key;
      deviceName = '';
      await loadDevices();
    } catch (err: unknown) {
      formError = err instanceof Error ? err.message : 'Failed to create device';
    } finally {
      formLoading = false;
    }
  }

  async function copyApiKey(key: string) {
    try {
      await navigator.clipboard.writeText(key);
      copied = true;
      setTimeout(() => copied = false, 2000);
    } catch {
      // clipboard unavailable
    }
  }
</script>

<div style="padding:32px;max-width:800px;margin:0 auto">
  <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:24px">
    <h1 style="font-size:1.6rem;color:#eee;margin:0">Devices</h1>
    <button
      onclick={() => { showForm = true; newApiKey = '' }}
      style="background:#7c5ce7;color:#fff;border:none;padding:10px 24px;border-radius:6px;cursor:pointer;font-size:.95rem"
      onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = '#6a4fd4'}
      onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = '#7c5ce7'}
    >
      Register Device
    </button>
  </div>

  {#if showForm}
    <div style="background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:24px;margin-bottom:24px">
      <h2 style="font-size:1.2rem;color:#eee;margin:0 0 16px">Register New Device</h2>
      <form onsubmit={handleCreate} style="display:flex;flex-direction:column;gap:14px">
        {#if formError}
          <p style="color:#e74c3c;margin:0;font-size:.9rem">{formError}</p>
        {/if}
        <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">
          Device Name *
          <input type="text" bind:value={deviceName} required
            style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none" />
        </label>
        <div style="display:flex;gap:12px">
          <button type="submit" disabled={formLoading}
            style="background:#7c5ce7;color:#fff;border:none;padding:10px 24px;border-radius:6px;cursor:pointer;font-size:.95rem;opacity:{formLoading ? 0.6 : 1}"
          >{formLoading ? 'Creating...' : 'Register'}</button>
          <button type="button" onclick={() => showForm = false}
            style="background:transparent;color:#aaa;border:1px solid #2a2a4a;padding:10px 24px;border-radius:6px;cursor:pointer;font-size:.95rem"
          >Cancel</button>
        </div>
      </form>

      {#if newApiKey}
        <div style="margin-top:20px;padding:16px;background:#0d0d1a;border:1px solid #4ade80;border-radius:8px">
          <p style="font-size:.85rem;color:#4ade80;margin:0 0 8px;font-weight:600">Device created! Save this API key — it won't be shown again.</p>
          <div style="display:flex;gap:8px;align-items:center">
            <code style="flex:1;padding:10px;background:#111;border-radius:6px;color:#4ade80;font-size:.85rem;word-break:break-all">{newApiKey}</code>
            <button
              onclick={() => copyApiKey(newApiKey)}
              style="background:#2a2a4a;color:#eee;border:none;padding:10px 16px;border-radius:6px;cursor:pointer;font-size:.85rem;white-space:nowrap"
              onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = '#3a3a5a'}
              onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = '#2a2a4a'}
            >{copied ? 'Copied!' : 'Copy'}</button>
          </div>
        </div>
      {/if}
    </div>
  {/if}

  {#if loading}
    <p style="color:#aaa;text-align:center;padding:40px">Loading...</p>
  {:else if error}
    <p style="color:#e74c3c;text-align:center;padding:40px">{error}</p>
  {:else if devices.length === 0}
    <p style="color:#aaa;text-align:center;padding:40px">No devices registered yet.</p>
  {:else}
    <div style="display:flex;flex-direction:column;gap:12px">
      {#each devices as device (device.id)}
        <div style="background:#1a1a2e;border:1px solid #2a2a4a;border-radius:10px;padding:20px">
          <div style="display:flex;justify-content:space-between;align-items:flex-start">
            <div>
              <h3 style="font-size:1.05rem;color:#eee;margin:0 0 6px">{device.name}</h3>
              <p style="font-size:.8rem;color:#888;margin:0">
                {new Date(device.created_at).toLocaleString()}
              </p>
            </div>
            <button
              onclick={() => copyApiKey(device.api_key)}
              style="background:#2a2a4a;color:#eee;border:none;padding:6px 14px;border-radius:6px;cursor:pointer;font-size:.8rem"
              onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = '#3a3a5a'}
              onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = '#2a2a4a'}
            >{copied ? 'Copied!' : 'Copy Key'}</button>
          </div>
          <code style="display:block;margin-top:10px;padding:8px 10px;background:#0d0d1a;border-radius:6px;color:#888;font-size:.8rem;word-break:break-all">{device.api_key.substring(0, 16)}...</code>
        </div>
      {/each}
    </div>
  {/if}
</div>
