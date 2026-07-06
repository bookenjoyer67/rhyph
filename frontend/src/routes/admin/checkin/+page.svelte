<script lang="ts">
  import { getCheckinStats } from '$lib/api';

  interface Stats {
    total_scans: number;
    currently_inside: number;
  }

  let listId = $state('');
  let loading = $state(false);
  let error = $state('');
  let stats = $state<Stats | null>(null);

  $effect(() => {});

  async function fetchStats(e: Event) {
    e.preventDefault();
    if (!listId) return;
    loading = true;
    error = '';
    stats = null;
    try {
      stats = await getCheckinStats(listId);
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Failed to fetch stats';
    } finally {
      loading = false;
    }
  }
</script>

<div style="padding:32px;max-width:700px;margin:0 auto">
  <h1 style="font-size:1.6rem;color:#eee;margin:0 0 24px">Checkin Dashboard</h1>

  <form onsubmit={fetchStats} style="background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:24px;margin-bottom:24px;display:flex;gap:12px;align-items:end">
    <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem;flex:1">
      Checkin List ID
      <input type="text" bind:value={listId} required
        style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none" />
    </label>
    <button type="submit" disabled={loading}
      style="background:#7c5ce7;color:#fff;border:none;padding:10px 24px;border-radius:6px;cursor:pointer;font-size:.95rem;white-space:nowrap;opacity:{loading ? 0.6 : 1}"
    >{loading ? 'Loading...' : 'Fetch Stats'}</button>
  </form>

  {#if loading}
    <p style="color:#aaa;text-align:center;padding:20px">Loading...</p>
  {:else if error}
    <p style="color:#e74c3c;text-align:center;padding:20px">{error}</p>
  {:else if stats}
    <div style="display:grid;grid-template-columns:1fr 1fr;gap:16px">
      <div style="background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:28px;text-align:center">
        <p style="font-size:2.5rem;font-weight:700;color:#7c5ce7;margin:0 0 8px">{stats.total_scans}</p>
        <p style="font-size:.9rem;color:#888;margin:0">Total Scans</p>
      </div>
      <div style="background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:28px;text-align:center">
        <p style="font-size:2.5rem;font-weight:700;color:#4ade80;margin:0 0 8px">{stats.currently_inside}</p>
        <p style="font-size:.9rem;color:#888;margin:0">Currently Inside</p>
      </div>
    </div>
  {:else}
    <p style="color:#888;text-align:center;padding:40px">Enter a checkin list ID to view stats.</p>
  {/if}
</div>
