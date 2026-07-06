<script lang="ts">
  import { page } from '$app/stores';
  import { getEvent, listItems, addToCart } from '$lib/api';
  import { goto } from '$app/navigation';

  const slug = $derived($page.params.slug);

  let event = $state<any>(null);
  let items = $state<any[]>([]);
  let loading = $state(true);
  let error = $state('');
  let primary = $state('#FF1493');
  let accent = $state('#00E676');

  let quantities = $state<Record<string,number>>({});
  let cartLoading = $state<Record<string,boolean>>({});
  let cartMsg = $state('');

  $effect(() => {
    Promise.all([
      getEvent('default', slug),
      listItems('default', slug),
    ]).then(([e, it]) => {
      event = e;
      items = it.filter((i: any) => i.active);
      loading = false;
    }).catch(err => { error = err.message; loading = false; });
  });

  function formatDate(iso: string): string {
    return new Date(iso).toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric', hour: 'numeric', minute: '2-digit' });
  }

  async function handleAdd(item: any) {
    const qty = quantities[item.id] || 0;
    if (qty <= 0) return;
    cartLoading[item.id] = true;
    try {
      await addToCart('default', slug, { item_id: item.id, quantity: qty });
      quantities[item.id] = 0;
      cartMsg = 'Added to cart!';
      setTimeout(() => cartMsg = '', 2000);
      // Update cart count
      const el = document.getElementById('cart-count');
      if (el) el.textContent = String(parseInt(el.textContent||'0') + qty);
    } catch (e) {
      cartMsg = 'Failed — try again';
    } finally {
      cartLoading[item.id] = false;
    }
  }

  function adj(id: string, delta: number) {
    const cur = quantities[id] || 0;
    quantities[id] = Math.max(0, cur + delta);
    quantities = { ...quantities };
  }
</script>

{#if loading}
  <div style="display:flex;align-items:center;justify-content:center;min-height:60vh">
    <div style="width:40px;height:40px;border:2px solid #1a1a2e;border-top-color:{primary};border-radius:50%;animation:spin 1s linear infinite"></div>
  </div>
{:else if error}
  <div style="text-align:center;padding:80px 24px"><p style="color:#e74c3c">{error}</p></div>
{:else if event}
  <div style="--nc-primary:{primary};--nc-accent:{accent}">
    <!-- Event hero -->
    <section style="padding:60px 24px 40px;text-align:center;max-width:800px;margin:0 auto">
      <div style="font-family:'Cinzel',serif;font-size:.75rem;text-transform:uppercase;letter-spacing:4px;color:{accent};margin-bottom:16px">{formatDate(event.date_from)}</div>
      <h1 style="font-family:'Cinzel',serif;font-size:clamp(2rem,5vw,3.5rem);font-weight:900;color:#fff;margin:0 0 16px;letter-spacing:2px;line-height:1.1;text-shadow:0 0 40px {primary}">{event.name}</h1>
      <p style="color:#888;font-size:1rem;margin:0 0 8px">{event.location}</p>
      {#if event.description}
        <div style="color:#666;font-size:.95rem;margin-top:20px;line-height:1.7;max-width:600px;margin-left:auto;margin-right:auto">{@html event.description}</div>
      {/if}
    </section>

    <!-- Tickets -->
    <section style="max-width:700px;margin:0 auto;padding:20px 24px 80px">
      <h2 style="font-family:'Cinzel',serif;font-size:1.3rem;text-align:center;color:#fff;margin:0 0 32px;letter-spacing:3px">TICKETS</h2>

      {#if items.length === 0}
        <p style="text-align:center;color:#666">Tickets coming soon.</p>
      {:else}
        <div style="display:flex;flex-direction:column;gap:12px">
          {#each items as item (item.id)}
            <div style="display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:16px;padding:24px;background:rgba(255,255,255,0.02);border:1px solid rgba(255,255,255,0.06);transition:border-color .2s"
                 onmouseenter={(e) => (e.currentTarget as HTMLElement).style.borderColor = primary}
                 onmouseleave={(e) => (e.currentTarget as HTMLElement).style.borderColor = 'rgba(255,255,255,0.06)'}
            >
              <div>
                <h3 style="font-family:'Cinzel',serif;font-weight:700;color:#fff;margin:0 0 4px;letter-spacing:1px;font-size:1rem">{item.name}</h3>
                {#if item.description}
                  <p style="color:#666;font-size:.8rem;margin:0 0 8px">{item.description}</p>
                {/if}
                <p style="color:{accent};font-weight:700;font-size:1.3rem;margin:0">${parseFloat(item.default_price || item.price || '0').toFixed(2)}</p>
              </div>

              <div style="display:flex;align-items:center;gap:10px">
                <button onclick={() => adj(item.id, -1)} disabled={(quantities[item.id]||0) <= 0}
                  style="width:36px;height:36px;border:1px solid #333;background:transparent;color:#fff;cursor:pointer;font-size:1.2rem;opacity:{(quantities[item.id]||0)<=0?0.3:1}">−</button>
                <span style="min-width:30px;text-align:center;font-weight:700;font-size:1.1rem">{quantities[item.id] || 0}</span>
                <button onclick={() => adj(item.id, 1)}
                  style="width:36px;height:36px;border:1px solid #333;background:transparent;color:#fff;cursor:pointer;font-size:1.2rem">+</button>
                <button onclick={() => handleAdd(item)} disabled={(quantities[item.id]||0) <= 0 || cartLoading[item.id]}
                  style="padding:10px 20px;background:{primary};color:#08080f;border:none;font-weight:700;cursor:pointer;text-transform:uppercase;letter-spacing:1px;font-size:.8rem;opacity:{(quantities[item.id]||0)<=0||cartLoading[item.id]?0.4:1}">
                  {cartLoading[item.id] ? '...' : 'Add'}
                </button>
              </div>
            </div>
          {/each}
        </div>

        {#if cartMsg}
          <p style="text-align:center;color:{cartMsg.includes('Failed')?'#e74c3c':'{accent}'};margin:16px 0 0;font-weight:600">{cartMsg}</p>
        {/if}

        <div style="text-align:center;margin-top:32px">
          <a href="/cart?org=default&event={slug}" style="display:inline-block;padding:16px 48px;background:transparent;border:2px solid {accent};color:{accent};text-decoration:none;font-weight:700;letter-spacing:2px;text-transform:uppercase;font-size:.9rem;transition:background .3s"
             onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = `${accent}11`}
             onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = 'transparent'}
          >View Cart →</a>
        </div>
      {/if}
    </section>
  </div>
{/if}

<style>
  @keyframes spin { to { transform: rotate(360deg); } }
</style>
