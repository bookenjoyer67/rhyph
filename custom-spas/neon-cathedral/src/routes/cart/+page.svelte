<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { getCart, removeFromCart, createOrder, listItems } from '$lib/api';

  const org = $derived($page.url.searchParams.get('org') || 'default');
  const eventSlug = $derived($page.url.searchParams.get('event') || '');

  let positions = $state<any[]>([]);
  let items = $state<any[]>([]);
  let loading = $state(true);
  let error = $state('');
  let email = $state('');
  let checkoutLoading = $state(false);
  let removing = $state<Record<string,boolean>>({});
  let primary = $state('#FF1493');
  let accent = $state('#00E676');

  $effect(() => {
    if (!eventSlug) { loading = false; error = 'No event specified.'; return; }
    Promise.all([
      getCart(org, eventSlug),
      listItems(org, eventSlug),
    ]).then(([cart, it]) => {
      positions = cart;
      items = it;
      loading = false;
    }).catch(e => { error = e.message; loading = false; });
  });

  function itemName(id: string): string {
    return items.find((i: any) => i.id === id)?.name || 'Ticket';
  }

  async function handleRemove(posId: string) {
    removing[posId] = true;
    try {
      await removeFromCart(org, eventSlug, posId);
      positions = positions.filter(p => p.id !== posId);
    } catch {}
    removing[posId] = false;
  }

  async function handleCheckout() {
    checkoutLoading = true;
    try {
      const result = await createOrder(org, eventSlug, { email: email || undefined });
      goto(`/orders/${result.code}?org=${org}&event=${eventSlug}`);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Checkout failed';
    } finally {
      checkoutLoading = false;
    }
  }

  const total = $derived(positions.reduce((s: number, p: any) => s + parseFloat(p.price), 0));
</script>

{#if loading}
  <div style="display:flex;align-items:center;justify-content:center;min-height:60vh">
    <div style="width:40px;height:40px;border:2px solid #1a1a2e;border-top-color:{primary};border-radius:50%;animation:spin 1s linear infinite"></div>
  </div>
{:else if error}
  <div style="text-align:center;padding:80px 24px"><p style="color:#e74c3c">{error}</p><a href="/" style="color:{primary}">← Home</a></div>
{:else if positions.length === 0}
  <div style="text-align:center;padding:80px 24px;max-width:500px;margin:0 auto">
    <h2 style="font-family:'Cinzel',serif;color:#fff;margin:0 0 16px;letter-spacing:2px">YOUR CART IS EMPTY</h2>
    <p style="color:#666;margin:0 0 24px">The ritual awaits. Select your offering.</p>
    <a href="/events/{eventSlug}" style="padding:14px 32px;background:{primary};color:#08080f;text-decoration:none;font-weight:700;letter-spacing:2px;text-transform:uppercase;font-size:.85rem">Browse Events</a>
  </div>
{:else}
  <section style="max-width:700px;margin:0 auto;padding:40px 24px 80px">
    <h1 style="font-family:'Cinzel',serif;font-size:2rem;font-weight:900;color:#fff;margin:0 0 40px;letter-spacing:3px;text-align:center">YOUR CART</h1>

    <div style="display:flex;flex-direction:column;gap:12px;margin-bottom:32px">
      {#each positions as pos (pos.id)}
        <div style="display:flex;align-items:center;justify-content:space-between;padding:20px 24px;background:rgba(255,255,255,0.02);border:1px solid rgba(255,255,255,0.06)">
          <div>
            <p style="color:#fff;font-weight:600;margin:0 0 4px">{itemName(pos.item_id)}</p>
            <p style="color:{accent};font-weight:700;margin:0">${parseFloat(pos.price).toFixed(2)}</p>
          </div>
          <button onclick={() => handleRemove(pos.id)} disabled={removing[pos.id]}
            style="background:none;border:1px solid #333;color:#e74c3c;padding:6px 12px;cursor:pointer;font-size:.85rem;opacity:{removing[pos.id]?0.4:1}">
            {removing[pos.id] ? '...' : 'Remove'}
          </button>
        </div>
      {/each}
    </div>

    <div style="display:flex;justify-content:space-between;align-items:center;padding:24px;background:rgba(255,255,255,0.02);border:1px solid rgba(255,255,255,0.06);margin-bottom:32px">
      <span style="font-family:'Cinzel',serif;font-size:1.2rem;color:#fff;letter-spacing:2px">TOTAL</span>
      <span style="font-size:1.8rem;font-weight:900;color:{accent}">${total.toFixed(2)}</span>
    </div>

    <div style="display:flex;flex-direction:column;gap:12px">
      <input type="email" bind:value={email} placeholder="Email (optional — for receipt)" style="padding:14px;background:#0d0d1a;border:1px solid #1a1a2e;color:#eee;font-size:.95rem;outline:none" />
      <button onclick={handleCheckout} disabled={checkoutLoading}
        style="padding:18px;background:{accent};color:#08080f;border:none;font-weight:700;letter-spacing:3px;text-transform:uppercase;font-size:.9rem;cursor:pointer;opacity:{checkoutLoading?0.5:1}">
        {checkoutLoading ? 'PROCESSING...' : 'COMPLETE PURCHASE'}
      </button>
    </div>
  </section>
{/if}

<style>@keyframes spin { to { transform: rotate(360deg); } }</style>
