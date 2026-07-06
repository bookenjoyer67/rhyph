<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { getCart, removeFromCart, createOrder, listItems } from '$lib/api';

  const org = $derived($page.url.searchParams.get('org') || 'default');
  const eventSlug = $derived($page.url.searchParams.get('event') || '');

  interface CartPosition {
    id: string;
    event_id: string;
    item_id: string;
    variation_id: string | null;
    session_key: string;
    price: string;
    expires: string;
    answers: unknown;
    seat_id: string | null;
    created_at: string;
  }

  interface Item {
    id: string;
    name: string;
    description: string;
    price: number;
    active: boolean;
    max_per_order: number;
  }

  let positions = $state<CartPosition[]>([]);
  let cartLoading = $state(true);
  let cartError = $state('');

  let items = $state<Item[]>([]);

  let email = $state('');
  let checkoutLoading = $state(false);
  let checkoutError = $state('');

  let removing = $state<Record<string, boolean>>({});

  $effect(() => {
    if (!eventSlug) {
      cartLoading = false;
      cartError = 'No event specified. Please select tickets from an event page.';
      return;
    }
    loadCart();
    loadItems();
  });

  async function loadCart() {
    cartLoading = true;
    cartError = '';
    try {
      positions = await getCart(org, eventSlug);
    } catch (err: unknown) {
      cartError = err instanceof Error ? err.message : 'Failed to load cart';
    } finally {
      cartLoading = false;
    }
  }

  async function loadItems() {
    try {
      items = await listItems(org, eventSlug);
    } catch {
      // non-critical
    }
  }

  function getItemName(itemId: string): string {
    return items.find(i => i.id === itemId)?.name || 'Ticket';
  }

  async function handleRemove(positionId: string) {
    removing = { ...removing, [positionId]: true };
    try {
      await removeFromCart(org, eventSlug, positionId);
      positions = positions.filter(p => p.id !== positionId);
    } catch (err: unknown) {
      // silently fail but keep the item in the list
    } finally {
      removing = { ...removing, [positionId]: false };
    }
  }

  async function handleCheckout() {
    checkoutLoading = true;
    checkoutError = '';
    try {
      const result = await createOrder(org, eventSlug, { email: email || undefined });
      goto(`/orders/${result.code}?org=${org}&event=${eventSlug}`);
    } catch (err: unknown) {
      checkoutError = err instanceof Error ? err.message : 'Failed to create order';
    } finally {
      checkoutLoading = false;
    }
  }

  const total = $derived(
    positions.reduce((sum, p) => sum + parseFloat(p.price), 0)
  );

  const groupedByItem = $derived(() => {
    const groups: Record<string, { positions: CartPosition[]; name: string; price: string }> = {};
    for (const p of positions) {
      if (!groups[p.item_id]) {
        groups[p.item_id] = { positions: [], name: getItemName(p.item_id), price: p.price };
      }
      groups[p.item_id].positions.push(p);
    }
    return Object.values(groups);
  });

  const inputStyle = 'padding:12px;border:1px solid #2a2a4a;border-radius:8px;background:#0d0d1a;color:#eee;outline:none;font-size:1rem;width:100%;box-sizing:border-box';
  const cardStyle = 'background:#1a1a2e;border:1px solid #2a2a4a;border-radius:16px;padding:24px';
  const accent = '#00E676';
  const positionCount = $derived(positions.length);
</script>

<div style="min-height:100vh;background:#111;color:#eee">
  <div style="max-width:800px;margin:0 auto;padding:48px 24px">
    <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:32px">
      <h1 style="font-size:2rem;font-weight:800;color:#fff;margin:0">
        Your Cart
        {#if positionCount > 0}
          <span style="font-size:1rem;font-weight:400;color:#888">({positionCount} {positionCount === 1 ? 'item' : 'items'})</span>
        {/if}
      </h1>
      {#if eventSlug}
        <a
          href="/events/{org}/{eventSlug}"
          style="color:{accent};text-decoration:none;font-size:.95rem"
        >&larr; Back to event</a>
      {/if}
    </div>

    {#if cartLoading}
      <p style="color:#aaa;text-align:center;padding:60px;font-size:1.1rem">Loading cart...</p>
    {:else if cartError}
      <div style="{cardStyle};text-align:center">
        <p style="color:#e74c3c;font-size:1.1rem;margin:0">{cartError}</p>
        <a href="/" style="display:inline-block;margin-top:16px;color:{accent};text-decoration:none;font-size:1rem">&larr; Home</a>
      </div>
    {:else if positions.length === 0}
      <div style="{cardStyle};text-align:center">
        <p style="color:#888;font-size:1.2rem;margin:0">Your cart is empty</p>
        {#if eventSlug}
          <a
            href="/events/{org}/{eventSlug}"
            style="display:inline-block;margin-top:16px;color:{accent};text-decoration:none;font-size:1rem"
          >Browse tickets</a>
        {/if}
      </div>
    {:else}
      <div style="display:flex;flex-direction:column;gap:12px;margin-bottom:24px">
        {#each groupedByItem() as group (group.positions[0].item_id)}
          <div style="{cardStyle};display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:12px">
            <div>
              <p style="font-size:1.1rem;font-weight:600;color:#fff;margin:0 0 4px">
                {group.name}
                <span style="font-weight:400;color:#888;font-size:.95rem">&times;{group.positions.length}</span>
              </p>
              <p style="font-size:1rem;color:{accent};margin:0;font-weight:600">
                {parseFloat(group.price).toFixed(2)} each
              </p>
            </div>

            <div style="display:flex;align-items:center;gap:8px">
              {#each group.positions as pos (pos.id)}
                <button
                  onclick={() => handleRemove(pos.id)}
                  disabled={removing[pos.id]}
                  style="padding:6px 12px;background:#2a1a1a;color:#e74c3c;border:1px solid #3a2020;border-radius:6px;cursor:pointer;font-size:.85rem;opacity:{removing[pos.id] ? 0.5 : 1}"
                >
                  {removing[pos.id] ? '...' : '&times;'}
                </button>
              {/each}
            </div>
          </div>
        {/each}
      </div>

      <div style="{cardStyle};margin-bottom:24px">
        <div style="display:flex;justify-content:space-between;align-items:center">
          <span style="font-size:1.3rem;font-weight:700;color:#fff">Total</span>
          <span style="font-size:1.5rem;font-weight:800;color:{accent}">{total.toFixed(2)}</span>
        </div>
      </div>

      <div style="{cardStyle}">
        <h2 style="font-size:1.2rem;font-weight:700;color:#fff;margin:0 0 16px">Checkout</h2>

        {#if checkoutError}
          <p style="color:#e74c3c;font-size:.95rem;margin:0 0 12px">{checkoutError}</p>
        {/if}

        <div style="margin-bottom:16px">
          <label style="display:flex;flex-direction:column;gap:6px;color:#aaa;font-size:.9rem">
            Email (optional)
            <input
              type="email"
              bind:value={email}
              placeholder="you@example.com"
              style={inputStyle}
            />
          </label>
        </div>

        <button
          onclick={handleCheckout}
          disabled={checkoutLoading}
          style="background:{accent};color:#111;border:none;padding:14px 32px;border-radius:10px;cursor:pointer;font-size:1.1rem;font-weight:700;width:100%;opacity:{checkoutLoading ? 0.6 : 1};transition:opacity .2s"
        >
          {checkoutLoading ? 'Creating order...' : 'Checkout'}
        </button>
      </div>
    {/if}
  </div>
</div>
