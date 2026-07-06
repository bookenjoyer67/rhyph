<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { getEvent, listItems, addToCart } from '$lib/api';

  const orgSlug = $derived($page.params.org);
  const slug = $derived($page.params.slug);

  interface EventData {
    id: string;
    slug: string;
    name: string;
    location: string;
    date_from: string;
    date_to: string | null;
    timezone: string;
    description: string;
    live: boolean;
    currency: string;
  }

  interface Item {
    id: string;
    name: string;
    description: string;
    price: number;
    active: boolean;
    max_per_order: number;
  }

  let event = $state<EventData | null>(null);
  let eventLoading = $state(true);
  let eventError = $state('');

  let items = $state<Item[]>([]);
  let itemsLoading = $state(true);
  let itemsError = $state('');

  let quantities = $state<Record<string, number>>({});
  let cartErrors = $state<Record<string, string>>({});
  let cartLoading = $state<Record<string, boolean>>({});

  $effect(() => {
    loadEvent();
    loadItems();
  });

  async function loadEvent() {
    eventLoading = true;
    eventError = '';
    try {
      event = await getEvent(orgSlug, slug);
    } catch (err: unknown) {
      eventError = err instanceof Error ? err.message : 'Failed to load event';
    } finally {
      eventLoading = false;
    }
  }

  async function loadItems() {
    itemsLoading = true;
    itemsError = '';
    try {
      items = await listItems(orgSlug, slug);
    } catch (err: unknown) {
      itemsError = err instanceof Error ? err.message : 'Failed to load tickets';
    } finally {
      itemsLoading = false;
    }
  }

  function getQuantity(itemId: string): number {
    return quantities[itemId] ?? 0;
  }

  function setQuantity(itemId: string, qty: number) {
    quantities = { ...quantities, [itemId]: Math.max(0, qty) };
  }

  async function handleAddToCart(item: Item) {
    const qty = getQuantity(item.id);
    if (qty <= 0) return;

    cartLoading = { ...cartLoading, [item.id]: true };
    cartErrors = { ...cartErrors, [item.id]: '' };
    try {
      await addToCart(orgSlug, slug, { item_id: item.id, quantity: qty });
      quantities = { ...quantities, [item.id]: 0 };
    } catch (err: unknown) {
      cartErrors = { ...cartErrors, [item.id]: err instanceof Error ? err.message : 'Failed to add to cart' };
    } finally {
      cartLoading = { ...cartLoading, [item.id]: false };
    }
  }

  function formatDate(iso: string): string {
    const d = new Date(iso);
    return d.toLocaleDateString('en-US', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric', hour: '2-digit', minute: '2-digit' });
  }

  const inputStyle = 'padding:12px 16px;border:1px solid #2a2a4a;border-radius:8px;background:#0d0d1a;color:#eee;outline:none;font-size:1rem;width:60px;text-align:center';
  const cardStyle = 'background:var(--rhyph-card-bg, #1a1a2e);border:1px solid var(--rhyph-card-border, #2a2a4a);border-radius:16px;padding:32px';
  const accent = 'var(--rhyph-accent, #00E676)';
  const primary = 'var(--rhyph-primary, #7c5ce7)';
</script>

<div style="min-height:100vh;background:#111;color:#eee">
  {#if eventLoading}
    <div style="display:flex;align-items:center;justify-content:center;min-height:100vh">
      <p style="color:#aaa;font-size:1.2rem">Loading event...</p>
    </div>
  {:else if eventError}
    <div style="display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:100vh;gap:24px">
      <p style="color:#e74c3c;font-size:1.2rem">{eventError}</p>
      <a href="/" style="color:{accent};text-decoration:none;font-size:1rem">&larr; Home</a>
    </div>
  {:else if event}
    <div style="max-width:900px;margin:0 auto;padding:48px 24px">
      <div style="text-align:center;margin-bottom:48px">
        <h1 style="font-size:3rem;font-weight:800;color:#fff;margin:0 0 16px;letter-spacing:-0.5px">{event.name}</h1>
        {#if event.location}
          <p style="font-size:1.15rem;color:#aaa;margin:0 0 8px">{event.location}</p>
        {/if}
        <p style="font-size:1.15rem;color:#888;margin:0">
          {formatDate(event.date_from)}
        </p>
        {#if event.description}
          <p style="font-size:1.05rem;color:#aaa;margin:24px 0 0;line-height:1.6;max-width:600px;margin-left:auto;margin-right:auto">{event.description}</p>
        {/if}
      </div>

      <h2 style="font-size:1.6rem;font-weight:700;color:#fff;margin:0 0 24px;text-align:center">Tickets</h2>

      {#if itemsLoading}
        <p style="color:#aaa;text-align:center;padding:40px;font-size:1.1rem">Loading tickets...</p>
      {:else if itemsError}
        <p style="color:#e74c3c;text-align:center;padding:40px;font-size:1.1rem">{itemsError}</p>
      {:else if items.length === 0}
        <div style="{cardStyle};text-align:center">
          <p style="color:#888;font-size:1.2rem;margin:0">Tickets coming soon</p>
          <p style="color:#666;font-size:1rem;margin:8px 0 0">Check back later for available tickets.</p>
        </div>
      {:else}
        <div style="display:flex;flex-direction:column;gap:20px">
          {#each items.filter(i => i.active) as item (item.id)}
            <div style="{cardStyle};display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:20px">
              <div style="flex:1;min-width:200px">
                <h3 style="font-size:1.3rem;font-weight:700;color:#fff;margin:0 0 4px">{item.name}</h3>
                {#if item.description}
                  <p style="font-size:.95rem;color:#888;margin:0 0 8px">{item.description}</p>
                {/if}
                <p style="font-size:1.5rem;font-weight:700;color:{accent};margin:0">
                  {item.price === 0 ? 'Free' : `${item.price.toFixed(2)}`}
                </p>
              </div>

              <div style="display:flex;align-items:center;gap:12px">
                <div style="display:flex;align-items:center;gap:4px">
                  <button
                    onclick={() => setQuantity(item.id, Math.max(0, getQuantity(item.id) - 1))}
                    disabled={getQuantity(item.id) <= 0}
                    style="padding:8px 14px;background:#2a2a4a;color:#eee;border:1px solid #3a3a5a;border-radius:6px;cursor:pointer;font-size:1.1rem;opacity:{getQuantity(item.id) <= 0 ? 0.3 : 1}"
                  >-</button>
                  <input
                    type="number"
                    min="0"
                    max={item.max_per_order}
                    value={getQuantity(item.id)}
                    oninput={(e) => setQuantity(item.id, parseInt((e.currentTarget as HTMLInputElement).value) || 0)}
                    style={inputStyle}
                  />
                  <button
                    onclick={() => setQuantity(item.id, Math.min(item.max_per_order, getQuantity(item.id) + 1))}
                    disabled={getQuantity(item.id) >= item.max_per_order}
                    style="padding:8px 14px;background:#2a2a4a;color:#eee;border:1px solid #3a3a5a;border-radius:6px;cursor:pointer;font-size:1.1rem;opacity:{getQuantity(item.id) >= item.max_per_order ? 0.3 : 1}"
                  >+</button>
                </div>

                <button
                  onclick={() => handleAddToCart(item)}
                  disabled={getQuantity(item.id) <= 0 || cartLoading[item.id]}
                  style="background:{accent};color:#111;border:none;padding:12px 28px;border-radius:8px;cursor:pointer;font-size:1rem;font-weight:700;white-space:nowrap;opacity:{getQuantity(item.id) <= 0 || cartLoading[item.id] ? 0.5 : 1};transition:opacity .2s"
                >
                  {cartLoading[item.id] ? 'Adding...' : 'Add to Cart'}
                </button>
              </div>

              {#if cartErrors[item.id]}
                <div style="width:100%">
                  <p style="color:#e74c3c;font-size:.9rem;margin:8px 0 0">{cartErrors[item.id]}</p>
                </div>
              {/if}
            </div>
          {/each}
        </div>

        <div style="text-align:center;margin-top:32px">
          <a
            href="/cart?org={orgSlug}&event={slug}"
            style="display:inline-block;background:{accent};color:#111;text-decoration:none;padding:14px 40px;border-radius:10px;font-size:1.15rem;font-weight:700;transition:opacity .2s"
          >View Cart &rarr;</a>
        </div>
      {/if}
    </div>
  {/if}
</div>
