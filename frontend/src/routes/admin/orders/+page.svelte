<script lang="ts">
  import { getOrderByCode } from '$lib/api';
  import { goto } from '$app/navigation';

  interface OrderData {
    order: {
      code: string;
      status: string;
      total: string;
      email: string | null;
      created_at: string;
    };
    positions: Array<{
      id: string;
      item_name: string;
      price: string;
      attendee_name: string | null;
      canceled: boolean;
    }>;
  }

  let orgSlug = $state('default');
  let eventSlug = $state('');
  let code = $state('');
  let loading = $state(false);
  let error = $state('');
  let order = $state<OrderData | null>(null);

  $effect(() => {
    if (!localStorage.getItem('rhyph_token')) {
      goto('/');
    }
  });

  async function lookupOrder(e: Event) {
    e.preventDefault();
    if (!eventSlug || !code) return;
    loading = true;
    error = '';
    order = null;
    try {
      order = await getOrderByCode(orgSlug, eventSlug, code);
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Failed to fetch order';
    } finally {
      loading = false;
    }
  }
</script>

<div style="padding:32px;max-width:800px;margin:0 auto">
  <h1 style="font-size:1.6rem;color:#eee;margin:0 0 24px">Order Lookup</h1>

  <form onsubmit={lookupOrder} style="background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:24px;margin-bottom:24px">
    <div style="display:grid;grid-template-columns:1fr 1fr 1fr;gap:12px;align-items:end">
      <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">
        Organizer Slug
        <input type="text" bind:value={orgSlug}
          style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none" />
      </label>
      <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">
        Event Slug *
        <input type="text" bind:value={eventSlug} required
          style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none" />
      </label>
      <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">
        Order Code *
        <input type="text" bind:value={code} required
          style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none" />
      </label>
    </div>
    <button type="submit" disabled={loading}
      style="margin-top:16px;background:#7c5ce7;color:#fff;border:none;padding:10px 28px;border-radius:6px;cursor:pointer;font-size:.95rem;opacity:{loading ? 0.6 : 1}"
    >{loading ? 'Looking up...' : 'Lookup'}</button>
  </form>

  {#if loading}
    <p style="color:#aaa;text-align:center;padding:20px">Loading...</p>
  {:else if error}
    <p style="color:#e74c3c;text-align:center;padding:20px">{error}</p>
  {:else if order}
    <div style="background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:24px">
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:20px">
        <div>
          <h2 style="font-size:1.2rem;color:#eee;margin:0 0 4px">Order {order.order.code}</h2>
          <p style="font-size:.85rem;color:#888;margin:0">
            {new Date(order.order.created_at).toLocaleString()}
            {order.order.email ? ` · ${order.order.email}` : ''}
          </p>
        </div>
        <div style="display:flex;gap:12px;align-items:center">
          <span style="padding:6px 16px;border-radius:20px;font-size:.8rem;font-weight:600;background:#2a2a4a;color:#888">
            {order.order.status}
          </span>
          <span style="font-size:1.2rem;font-weight:700;color:#4ade80">{order.order.total}</span>
        </div>
      </div>

      <h3 style="font-size:1rem;color:#aaa;margin:0 0 12px">Positions ({order.positions.length})</h3>
      {#if order.positions.length === 0}
        <p style="color:#888;margin:0">No positions</p>
      {:else}
        <div style="display:flex;flex-direction:column;gap:8px">
          {#each order.positions as pos (pos.id)}
            <div style="display:flex;justify-content:space-between;align-items:center;padding:12px;background:#0d0d1a;border-radius:8px">
              <div>
                <p style="font-size:.9rem;color:#eee;margin:0">{pos.item_name}</p>
                {#if pos.attendee_name}
                  <p style="font-size:.8rem;color:#888;margin:2px 0 0">{pos.attendee_name}</p>
                {/if}
              </div>
              <div style="display:flex;gap:12px;align-items:center">
                <span style="font-size:.9rem;color:#4ade80">{pos.price}</span>
                {#if pos.canceled}
                  <span style="padding:2px 10px;background:#4a2020;color:#f87171;border-radius:20px;font-size:.75rem">Canceled</span>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
