<script lang="ts">
  import { page } from '$app/stores';
  import { getOrderByCode } from '$lib/api';

  const code = $derived($page.params.code);
  const org = $derived($page.url.searchParams.get('org') || 'default');
  const eventSlug = $derived($page.url.searchParams.get('event') || '');

  interface OrderInfo {
    id: string;
    event_id: string;
    code: string;
    status: string;
    secret: string;
    email: string | null;
    phone: string | null;
    locale: string;
    total: string;
    datetime: string;
    expires: string | null;
    payment_provider: string | null;
    payment_state: string;
    testmode: boolean;
    require_approval: boolean;
    valid_if_pending: boolean;
    sales_channel: string;
    created_at: string;
    updated_at: string;
  }

  interface Position {
    id: string;
    order_id: string;
    positionid: number;
    item_id: string;
    variation_id: string | null;
    price: string;
    secret: string;
    attendee_name: string | null;
    attendee_email: string | null;
    answers: unknown;
    seat_id: string | null;
    pseudonymization_id: string;
    canceled: boolean;
    valid_from: string | null;
    valid_until: string | null;
    item_name: string;
  }

  interface OrderData {
    order: OrderInfo;
    positions: Position[];
  }

  let orderData = $state<OrderData | null>(null);
  let loading = $state(true);
  let error = $state('');

  $effect(() => {
    if (!eventSlug) {
      loading = false;
      error = 'Missing event information. Try accessing this page from your order confirmation email.';
      return;
    }
    loadOrder();
  });

  async function loadOrder() {
    loading = true;
    error = '';
    try {
      orderData = await getOrderByCode(org, eventSlug, code);
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Failed to load order';
    } finally {
      loading = false;
    }
  }

  function statusColor(status: string): string {
    switch (status) {
      case 'paid': return '#00E676';
      case 'pending': return '#f39c12';
      case 'canceled': return '#e74c3c';
      case 'expired': return '#e74c3c';
      default: return '#888';
    }
  }

  function formatDate(iso: string): string {
    const d = new Date(iso);
    return d.toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric', hour: '2-digit', minute: '2-digit' });
  }

  const cardStyle = 'background:var(--rhyph-card-bg, #1a1a2e);border:1px solid var(--rhyph-card-border, #2a2a4a);border-radius:16px;padding:28px';
  const accent = 'var(--rhyph-accent, #00E676)';
</script>

<div style="min-height:100vh;background:#111;color:#eee">
  <div style="max-width:800px;margin:0 auto;padding:48px 24px">
    {#if loading}
      <div style="display:flex;align-items:center;justify-content:center;min-height:60vh">
        <p style="color:#aaa;font-size:1.2rem">Loading order...</p>
      </div>
    {:else if error}
      <div style="display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:60vh;gap:16px">
        <p style="color:#e74c3c;font-size:1.2rem;margin:0">{error}</p>
        <a href="/" style="color:{accent};text-decoration:none;font-size:1rem">&larr; Home</a>
      </div>
    {:else if orderData}
      <div style="text-align:center;margin-bottom:40px">
        <div style="font-size:1rem;color:#888;margin-bottom:8px">Order</div>
        <h1 style="font-size:2.5rem;font-weight:800;color:#fff;margin:0 0 12px;letter-spacing:2px;font-family:monospace">
          {orderData.order.code}
        </h1>
        <span style="display:inline-block;padding:6px 20px;border-radius:20px;font-size:.9rem;font-weight:700;background:{statusColor(orderData.order.status)}22;color:{statusColor(orderData.order.status)};border:1px solid {statusColor(orderData.order.status)}44">
          {orderData.order.status.toUpperCase()}
        </span>
      </div>

      <div style="{cardStyle};margin-bottom:24px">
        <h2 style="font-size:1.1rem;font-weight:700;color:#fff;margin:0 0 16px">Order Details</h2>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:12px">
          <div>
            <p style="color:#888;font-size:.85rem;margin:0 0 4px">Date</p>
            <p style="color:#eee;font-size:.95rem;margin:0">{formatDate(orderData.order.created_at)}</p>
          </div>
          <div>
            <p style="color:#888;font-size:.85rem;margin:0 0 4px">Email</p>
            <p style="color:#eee;font-size:.95rem;margin:0">{orderData.order.email || 'Not provided'}</p>
          </div>
          <div>
            <p style="color:#888;font-size:.85rem;margin:0 0 4px">Total</p>
            <p style="color:{accent};font-size:.95rem;margin:0;font-weight:700">{parseFloat(orderData.order.total).toFixed(2)}</p>
          </div>
          <div>
            <p style="color:#888;font-size:.85rem;margin:0 0 4px">Positions</p>
            <p style="color:#eee;font-size:.95rem;margin:0">{orderData.positions.length}</p>
          </div>
        </div>
      </div>

      <h2 style="font-size:1.2rem;font-weight:700;color:#fff;margin:0 0 16px">Tickets</h2>

      <div style="display:flex;flex-direction:column;gap:12px;margin-bottom:32px">
        {#each orderData.positions.filter(p => !p.canceled) as pos (pos.id)}
          <div style="{cardStyle};display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:12px">
            <div>
              <p style="font-size:1.05rem;font-weight:600;color:#fff;margin:0 0 4px">{pos.item_name}</p>
              <div style="display:flex;gap:16px;flex-wrap:wrap">
                {#if pos.attendee_name}
                  <p style="font-size:.9rem;color:#aaa;margin:0">Attendee: {pos.attendee_name}</p>
                {/if}
                {#if pos.seat_id}
                  <p style="font-size:.9rem;color:#aaa;margin:0">Seat: {pos.seat_id}</p>
                {/if}
              </div>
              <p style="font-size:.8rem;color:#666;margin:4px 0 0;font-family:monospace">#{pos.positionid}</p>
            </div>
            <p style="font-size:1.1rem;font-weight:700;color:{accent};margin:0">
              {parseFloat(pos.price).toFixed(2)}
            </p>
          </div>
        {/each}

        {#each orderData.positions.filter(p => p.canceled) as pos (pos.id)}
          <div style="{cardStyle};display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:12px;opacity:0.5">
            <div>
              <p style="font-size:1.05rem;font-weight:600;color:#888;margin:0 0 4px;text-decoration:line-through">{pos.item_name}</p>
              <p style="font-size:.85rem;color:#e74c3c;margin:0">Canceled</p>
            </div>
            <p style="font-size:1.1rem;font-weight:700;color:#666;margin:0;text-decoration:line-through">
              {parseFloat(pos.price).toFixed(2)}
            </p>
          </div>
        {/each}
      </div>

      <div style="{cardStyle};display:flex;align-items:center;justify-content:space-between">
        <span style="font-size:1.2rem;font-weight:700;color:#fff">Total</span>
        <span style="font-size:1.4rem;font-weight:800;color:{accent}">{parseFloat(orderData.order.total).toFixed(2)}</span>
      </div>

      <div style="text-align:center;margin-top:32px">
        <button
          onclick={() => {}}
          style="background:transparent;color:{accent};border:2px solid {accent};padding:12px 32px;border-radius:10px;cursor:pointer;font-size:1rem;font-weight:700;opacity:0.7"
          disabled
        >Download Tickets (coming soon)</button>
      </div>
    {/if}
  </div>
</div>
