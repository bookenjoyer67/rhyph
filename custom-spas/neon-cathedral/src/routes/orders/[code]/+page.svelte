<script lang="ts">
  import { page } from '$app/stores';
  import { getOrderByCode } from '$lib/api';

  const code = $derived($page.params.code);
  const org = $derived($page.url.searchParams.get('org') || 'default');
  const eventSlug = $derived($page.url.searchParams.get('event') || '');

  let orderData = $state<any>(null);
  let loading = $state(true);
  let error = $state('');
  let primary = $state('#FF1493');
  let accent = $state('#00E676');

  $effect(() => {
    if (!eventSlug) { loading = false; error = 'Missing event info.'; return; }
    getOrderByCode(org, eventSlug, code)
      .then(o => { orderData = o; loading = false; })
      .catch(e => { error = e.message; loading = false; });
  });

  function statusColor(s: string): string {
    switch(s) { case 'paid': return accent; case 'pending': return '#f39c12'; default: return '#e74c3c'; }
  }
  function fmt(iso: string): string {
    return new Date(iso).toLocaleDateString('en-US', { month:'long',day:'numeric',hour:'2-digit',minute:'2-digit' });
  }
</script>

{#if loading}
  <div style="display:flex;align-items:center;justify-content:center;min-height:60vh">
    <div style="width:40px;height:40px;border:2px solid #1a1a2e;border-top-color:{primary};border-radius:50%;animation:spin 1s linear infinite"></div>
  </div>
{:else if error}
  <div style="text-align:center;padding:80px 24px"><p style="color:#e74c3c">{error}</p><a href="/" style="color:{primary}">← Home</a></div>
{:else if orderData}
  <section style="max-width:600px;margin:0 auto;padding:40px 24px 80px;text-align:center">
    <div style="font-family:'Cinzel',serif;font-size:.7rem;text-transform:uppercase;letter-spacing:4px;color:#888;margin-bottom:8px">Order Confirmed</div>
    <h1 style="font-family:'Cinzel',serif;font-size:2.5rem;font-weight:900;color:#fff;margin:0 0 8px;letter-spacing:3px;text-shadow:0 0 40px {primary}">{orderData.order.code}</h1>
    <span style="display:inline-block;padding:6px 20px;border:1px solid {statusColor(orderData.order.status)};color:{statusColor(orderData.order.status)};font-size:.8rem;letter-spacing:2px;text-transform:uppercase;margin:16px 0 32px">{orderData.order.status}</span>

    <div style="text-align:left;padding:24px;background:rgba(255,255,255,0.02);border:1px solid rgba(255,255,255,0.06);margin-bottom:24px">
      <div style="display:flex;justify-content:space-between;padding:8px 0;border-bottom:1px solid rgba(255,255,255,0.04);color:#888;font-size:.85rem">
        <span>Date</span><span style="color:#eee">{fmt(orderData.order.created_at)}</span>
      </div>
      <div style="display:flex;justify-content:space-between;padding:8px 0;border-bottom:1px solid rgba(255,255,255,0.04);color:#888;font-size:.85rem">
        <span>Email</span><span style="color:#eee">{orderData.order.email || '—'}</span>
      </div>
      <div style="display:flex;justify-content:space-between;padding:8px 0;color:#888;font-size:.85rem">
        <span>Tickets</span><span style="color:#eee">{orderData.positions?.length || 0}</span>
      </div>
    </div>

    <h2 style="font-family:'Cinzel',serif;color:#fff;margin:32px 0 20px;letter-spacing:2px;text-align:left">TICKETS</h2>
    <div style="display:flex;flex-direction:column;gap:8px;text-align:left">
      {#each orderData.positions?.filter((p:any) => !p.canceled) as pos}
        <div style="display:flex;justify-content:space-between;align-items:center;padding:16px 20px;background:rgba(255,255,255,0.02);border:1px solid rgba(255,255,255,0.06)">
          <div>
            <p style="color:#fff;font-weight:600;margin:0 0 2px">{pos.item_name}</p>
            {#if pos.attendee_name}<p style="color:#888;font-size:.8rem;margin:0">{pos.attendee_name}</p>{/if}
          </div>
          <span style="color:{accent};font-weight:700">${parseFloat(pos.price).toFixed(2)}</span>
        </div>
      {/each}
    </div>

    <div style="display:flex;justify-content:space-between;padding:20px;margin-top:16px;background:rgba(255,255,255,0.02);border:1px solid rgba(255,255,255,0.06)">
      <span style="font-family:'Cinzel',serif;color:#fff;letter-spacing:2px">TOTAL PAID</span>
      <span style="font-size:1.4rem;font-weight:900;color:{accent}">${parseFloat(orderData.order.total).toFixed(2)}</span>
    </div>

    <a href="/" style="display:inline-block;margin-top:40px;color:{primary};text-decoration:none;letter-spacing:2px;font-size:.85rem">← RETURN TO THE CATHEDRAL</a>
  </section>
{/if}

<style>@keyframes spin { to { transform: rotate(360deg); } }</style>
