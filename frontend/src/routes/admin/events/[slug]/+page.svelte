<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import {
    getEvent,
    listItems,
    createItem,
    updateItem,
    listQuotas,
    createQuota,
    deleteQuota,
    listCheckinLists,
    createCheckinList,
    updateEvent,
  } from '$lib/api';

  const slug = $derived($page.params.slug);
  const orgSlug = 'default';

  interface EventData {
    id: string;
    slug: string;
    name: string;
    location: string;
    date_from: string;
    date_to: string | null;
    timezone: string;
    presale_start: string | null;
    presale_end: string | null;
    live: boolean;
    currency: string;
    checkout_text: string;
  }

  interface Item {
    id: string;
    name: string;
    description: string;
    price: number;
    active: boolean;
    max_per_order: number;
  }

  interface Quota {
    id: string;
    name: string;
    size: number;
    item_ids: string[];
  }

  interface CheckinList {
    id: string;
    name: string;
    all_products: boolean;
  }

  let event = $state<EventData | null>(null);
  let eventLoading = $state(true);
  let eventError = $state('');

  let tab = $state<'tickets' | 'quotas' | 'checkin' | 'settings'>('tickets');

  const tabs: { key: typeof tab; label: string }[] = [
    { key: 'tickets', label: 'Tickets' },
    { key: 'quotas', label: 'Quotas' },
    { key: 'checkin', label: 'Check-in' },
    { key: 'settings', label: 'Settings' },
  ];

  $effect(() => {
    if (!localStorage.getItem('rhyph_token')) {
      goto('/');
      return;
    }
    loadEvent();
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

  // Tickets
  let items = $state<Item[]>([]);
  let itemsLoading = $state(false);
  let itemsError = $state('');

  let itemForm = $state({ name: '', price: 0, description: '', max_per_order: 10 });
  let itemFormLoading = $state(false);
  let itemFormError = $state('');

  async function loadItems() {
    itemsLoading = true;
    itemsError = '';
    try {
      items = await listItems(orgSlug, slug);
    } catch (err: unknown) {
      itemsError = err instanceof Error ? err.message : 'Failed to load items';
    } finally {
      itemsLoading = false;
    }
  }

  async function handleCreateItem(e: Event) {
    e.preventDefault();
    itemFormLoading = true;
    itemFormError = '';
    try {
      await createItem(orgSlug, slug, { ...itemForm });
      itemForm = { name: '', price: 0, description: '', max_per_order: 10 };
      await loadItems();
    } catch (err: unknown) {
      itemFormError = err instanceof Error ? err.message : 'Failed to create item';
    } finally {
      itemFormLoading = false;
    }
  }

  async function toggleActive(item: Item) {
    try {
      await updateItem(orgSlug, slug, item.id, { active: !item.active });
      await loadItems();
    } catch (err: unknown) {
      itemsError = err instanceof Error ? err.message : 'Failed to update item';
    }
  }

  // Quotas
  let quotas = $state<Quota[]>([]);
  let quotasLoading = $state(false);
  let quotasError = $state('');

  let quotaForm = $state({ name: '', size: 0, item_ids: [] as string[] });
  let quotaFormLoading = $state(false);
  let quotaFormError = $state('');

  async function loadQuotas() {
    quotasLoading = true;
    quotasError = '';
    try {
      quotas = await listQuotas(orgSlug, slug);
    } catch (err: unknown) {
      quotasError = err instanceof Error ? err.message : 'Failed to load quotas';
    } finally {
      quotasLoading = false;
    }
  }

  async function handleCreateQuota(e: Event) {
    e.preventDefault();
    quotaFormLoading = true;
    quotaFormError = '';
    try {
      await createQuota(orgSlug, slug, { ...quotaForm });
      quotaForm = { name: '', size: 0, item_ids: [] };
      await loadQuotas();
    } catch (err: unknown) {
      quotaFormError = err instanceof Error ? err.message : 'Failed to create quota';
    } finally {
      quotaFormLoading = false;
    }
  }

  async function handleDeleteQuota(id: string) {
    try {
      await deleteQuota(orgSlug, slug, id);
      await loadQuotas();
    } catch (err: unknown) {
      quotasError = err instanceof Error ? err.message : 'Failed to delete quota';
    }
  }

  function toggleQuotaItem(itemId: string) {
    if (quotaForm.item_ids.includes(itemId)) {
      quotaForm.item_ids = quotaForm.item_ids.filter((id) => id !== itemId);
    } else {
      quotaForm.item_ids = [...quotaForm.item_ids, itemId];
    }
  }

  // Checkin Lists
  let checkinLists = $state<CheckinList[]>([]);
  let checkinLoading = $state(false);
  let checkinError = $state('');

  let checkinForm = $state({ name: '', all_products: false });
  let checkinFormLoading = $state(false);
  let checkinFormError = $state('');

  async function loadCheckinLists() {
    checkinLoading = true;
    checkinError = '';
    try {
      checkinLists = await listCheckinLists(orgSlug, slug);
    } catch (err: unknown) {
      checkinError = err instanceof Error ? err.message : 'Failed to load checkin lists';
    } finally {
      checkinLoading = false;
    }
  }

  async function handleCreateCheckinList(e: Event) {
    e.preventDefault();
    checkinFormLoading = true;
    checkinFormError = '';
    try {
      await createCheckinList(orgSlug, slug, { ...checkinForm });
      checkinForm = { name: '', all_products: false };
      await loadCheckinLists();
    } catch (err: unknown) {
      checkinFormError = err instanceof Error ? err.message : 'Failed to create checkin list';
    } finally {
      checkinFormLoading = false;
    }
  }

  // Settings
  let settingsForm = $state({
    name: '',
    location: '',
    date_from: '',
    timezone: 'UTC',
    presale_start: '',
    presale_end: '',
    live: false,
  });
  let settingsLoading = $state(false);
  let settingsError = $state('');

  $effect(() => {
    if (event) {
      settingsForm = {
        name: event.name,
        location: event.location || '',
        date_from: formatDatetimeLocal(event.date_from),
        timezone: event.timezone,
        presale_start: event.presale_start ? formatDatetimeLocal(event.presale_start) : '',
        presale_end: event.presale_end ? formatDatetimeLocal(event.presale_end) : '',
        live: event.live,
      };
    }
  });

  function formatDatetimeLocal(iso: string): string {
    const d = new Date(iso);
    const pad = (n: number) => n.toString().padStart(2, '0');
    return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}`;
  }

  async function handleUpdateEvent(e: Event) {
    e.preventDefault();
    settingsLoading = true;
    settingsError = '';
    try {
      const payload: Record<string, unknown> = {
        name: settingsForm.name,
        location: settingsForm.location,
        date_from: settingsForm.date_from ? new Date(settingsForm.date_from).toISOString() : undefined,
        timezone: settingsForm.timezone,
        presale_start: settingsForm.presale_start ? new Date(settingsForm.presale_start).toISOString() : null,
        presale_end: settingsForm.presale_end ? new Date(settingsForm.presale_end).toISOString() : null,
        live: settingsForm.live,
      };
      await updateEvent(orgSlug, slug, payload);
      await loadEvent();
    } catch (err: unknown) {
      settingsError = err instanceof Error ? err.message : 'Failed to update event';
    } finally {
      settingsLoading = false;
    }
  }

  function ensureTabData() {
    if (tab === 'tickets' && items.length === 0 && !itemsLoading) loadItems();
    if (tab === 'quotas' && quotas.length === 0 && !quotasLoading) loadQuotas();
    if (tab === 'checkin' && checkinLists.length === 0 && !checkinLoading) loadCheckinLists();
  }

  $effect(() => {
    tab;
    ensureTabData();
  });

  const inputStyle = 'padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none;font-size:.9rem';
  const labelStyle = 'display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem';
  const cardStyle = 'background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:24px;margin-bottom:16px';
  const btnPrimary = 'background:#7c5ce7;color:#fff;border:none;padding:10px 24px;border-radius:6px;cursor:pointer;font-size:.95rem';
  const btnDanger = 'background:#c0392b;color:#fff;border:none;padding:6px 14px;border-radius:6px;cursor:pointer;font-size:.85rem';
  const btnGhost = 'background:transparent;color:#aaa;border:1px solid #2a2a4a;padding:10px 24px;border-radius:6px;cursor:pointer;font-size:.95rem';
</script>

<div style="padding:32px;max-width:1000px;margin:0 auto">
  <div style="display:flex;align-items:center;gap:16px;margin-bottom:24px">
    <button
      onclick={() => goto('/admin/events')}
      style="background:transparent;color:#aaa;border:none;cursor:pointer;font-size:1.3rem;padding:0 4px"
    >&larr;</button>
    <h1 style="font-size:1.6rem;color:#eee;margin:0">
      {#if eventLoading}
        Loading...
      {:else if event}
        {event.name}
      {:else}
        Event
      {/if}
    </h1>
  </div>

  {#if eventLoading}
    <p style="color:#aaa;text-align:center;padding:40px">Loading event...</p>
  {:else if eventError}
    <p style="color:#e74c3c;text-align:center;padding:40px">{eventError}</p>
    <div style="text-align:center">
      <button onclick={() => goto('/admin/events')} style={btnGhost}>Back to Events</button>
    </div>
  {:else}
    <div style="display:flex;gap:4px;margin-bottom:24px;border-bottom:2px solid #2a2a4a">
      {#each tabs as t}
        <button
          onclick={() => tab = t.key}
          style="padding:10px 20px;border:none;background:transparent;color:{tab === t.key ? '#7c5ce7' : '#888'};cursor:pointer;font-size:.95rem;border-bottom:2px solid {tab === t.key ? '#7c5ce7' : 'transparent'};margin-bottom:-2px;transition:color .2s"
        >{t.label}</button>
      {/each}
    </div>

    {#if tab === 'tickets'}
      <div style={cardStyle}>
        <h2 style="font-size:1.2rem;color:#eee;margin:0 0 16px">Create Ticket</h2>
        <form onsubmit={handleCreateItem} style="display:flex;flex-direction:column;gap:14px">
          {#if itemFormError}
            <p style="color:#e74c3c;margin:0;font-size:.9rem">{itemFormError}</p>
          {/if}
          <div style="display:grid;grid-template-columns:2fr 1fr 1fr;gap:12px">
            <label style={labelStyle}>
              Name *
              <input type="text" bind:value={itemForm.name} required style={inputStyle} />
            </label>
            <label style={labelStyle}>
              Price *
              <input type="number" bind:value={itemForm.price} step="0.01" min="0" required style={inputStyle} />
            </label>
            <label style={labelStyle}>
              Max per Order
              <input type="number" bind:value={itemForm.max_per_order} min="1" style={inputStyle} />
            </label>
          </div>
          <label style={labelStyle}>
            Description
            <input type="text" bind:value={itemForm.description} style={inputStyle} />
          </label>
          <button type="submit" disabled={itemFormLoading}
            style="{btnPrimary};opacity:{itemFormLoading ? 0.6 : 1}"
          >{itemFormLoading ? 'Creating...' : 'Create'}</button>
        </form>
      </div>

      {#if itemsLoading}
        <p style="color:#aaa;text-align:center;padding:20px">Loading tickets...</p>
      {:else if itemsError}
        <p style="color:#e74c3c;text-align:center;padding:20px">{itemsError}</p>
      {:else if items.length === 0}
        <p style="color:#888;text-align:center;padding:20px">No tickets yet.</p>
      {:else}
        <div style={cardStyle}>
          <table style="width:100%;border-collapse:collapse">
            <thead>
              <tr style="text-align:left;border-bottom:1px solid #2a2a4a">
                <th style="padding:10px 12px;font-size:.85rem;color:#888;font-weight:600">Name</th>
                <th style="padding:10px 12px;font-size:.85rem;color:#888;font-weight:600">Price</th>
                <th style="padding:10px 12px;font-size:.85rem;color:#888;font-weight:600">Max/Order</th>
                <th style="padding:10px 12px;font-size:.85rem;color:#888;font-weight:600">Active</th>
              </tr>
            </thead>
            <tbody>
              {#each items as item (item.id)}
                <tr style="border-bottom:1px solid #2a2a4a">
                  <td style="padding:12px;color:#eee;font-size:.95rem">
                    <div>{item.name}</div>
                    {#if item.description}
                      <div style="color:#666;font-size:.8rem;margin-top:2px">{item.description}</div>
                    {/if}
                  </td>
                  <td style="padding:12px;color:#eee;font-size:.95rem">{item.price.toFixed(2)}</td>
                  <td style="padding:12px;color:#eee;font-size:.95rem">{item.max_per_order}</td>
                  <td style="padding:12px">
                    <button
                      onclick={() => toggleActive(item)}
                      style="padding:4px 14px;border-radius:20px;border:none;cursor:pointer;font-size:.8rem;font-weight:600;background:{item.active ? '#1a472a' : '#2a2a4a'};color:{item.active ? '#4ade80' : '#888'}"
                    >{item.active ? 'Active' : 'Inactive'}</button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}

    {:else if tab === 'quotas'}
      <div style={cardStyle}>
        <h2 style="font-size:1.2rem;color:#eee;margin:0 0 16px">Create Quota</h2>
        <form onsubmit={handleCreateQuota} style="display:flex;flex-direction:column;gap:14px">
          {#if quotaFormError}
            <p style="color:#e74c3c;margin:0;font-size:.9rem">{quotaFormError}</p>
          {/if}
          <div style="display:grid;grid-template-columns:2fr 1fr;gap:12px">
            <label style={labelStyle}>
              Name *
              <input type="text" bind:value={quotaForm.name} required style={inputStyle} />
            </label>
            <label style={labelStyle}>
              Size *
              <input type="number" bind:value={quotaForm.size} min="0" required style={inputStyle} />
            </label>
          </div>
          <div>
            <p style="color:#aaa;font-size:.85rem;margin:0 0 8px">Items</p>
            <div style="display:flex;flex-wrap:wrap;gap:8px">
              {#if items.length === 0 && !itemsLoading}
                <p style="color:#666;font-size:.85rem;margin:0">No items available. Create tickets first.</p>
              {:else}
                {#each items as item (item.id)}
                  <label style="display:flex;align-items:center;gap:6px;padding:6px 12px;border:1px solid {quotaForm.item_ids.includes(item.id) ? '#7c5ce7' : '#2a2a4a'};border-radius:6px;cursor:pointer;color:#ccc;font-size:.85rem;background:{quotaForm.item_ids.includes(item.id) ? '#1a1530' : 'transparent'}">
                    <input
                      type="checkbox"
                      checked={quotaForm.item_ids.includes(item.id)}
                      onchange={() => toggleQuotaItem(item.id)}
                      style="accent-color:#7c5ce7"
                    />
                    {item.name}
                  </label>
                {/each}
              {/if}
            </div>
          </div>
          <button type="submit" disabled={quotaFormLoading}
            style="{btnPrimary};opacity:{quotaFormLoading ? 0.6 : 1}"
          >{quotaFormLoading ? 'Creating...' : 'Create'}</button>
        </form>
      </div>

      {#if quotasLoading}
        <p style="color:#aaa;text-align:center;padding:20px">Loading quotas...</p>
      {:else if quotasError}
        <p style="color:#e74c3c;text-align:center;padding:20px">{quotasError}</p>
      {:else if quotas.length === 0}
        <p style="color:#888;text-align:center;padding:20px">No quotas yet.</p>
      {:else}
        <div style={cardStyle}>
          <table style="width:100%;border-collapse:collapse">
            <thead>
              <tr style="text-align:left;border-bottom:1px solid #2a2a4a">
                <th style="padding:10px 12px;font-size:.85rem;color:#888;font-weight:600">Name</th>
                <th style="padding:10px 12px;font-size:.85rem;color:#888;font-weight:600">Size</th>
                <th style="padding:10px 12px;font-size:.85rem;color:#888;font-weight:600">Items</th>
                <th style="padding:10px 12px;font-size:.85rem;color:#888;font-weight:600"></th>
              </tr>
            </thead>
            <tbody>
              {#each quotas as quota (quota.id)}
                <tr style="border-bottom:1px solid #2a2a4a">
                  <td style="padding:12px;color:#eee;font-size:.95rem">{quota.name}</td>
                  <td style="padding:12px;color:#eee;font-size:.95rem">{quota.size}</td>
                  <td style="padding:12px;color:#aaa;font-size:.85rem">
                    {quota.item_ids.map((id: string) => items.find((i: Item) => i.id === id)?.name || id).join(', ') || 'All'}
                  </td>
                  <td style="padding:12px;text-align:right">
                    <button
                      onclick={() => handleDeleteQuota(quota.id)}
                      style={btnDanger}
                      onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = '#e74c3c'}
                      onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = '#c0392b'}
                    >Delete</button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}

    {:else if tab === 'checkin'}
      <div style={cardStyle}>
        <h2 style="font-size:1.2rem;color:#eee;margin:0 0 16px">Create Check-in List</h2>
        <form onsubmit={handleCreateCheckinList} style="display:flex;flex-direction:column;gap:14px">
          {#if checkinFormError}
            <p style="color:#e74c3c;margin:0;font-size:.9rem">{checkinFormError}</p>
          {/if}
          <div style="display:grid;grid-template-columns:1fr auto;gap:12px;align-items:end">
            <label style={labelStyle}>
              Name *
              <input type="text" bind:value={checkinForm.name} required style={inputStyle} />
            </label>
            <label style="display:flex;align-items:center;gap:8px;color:#aaa;font-size:.85rem;padding-bottom:1px">
              <input
                type="checkbox"
                bind:checked={checkinForm.all_products}
                style="accent-color:#7c5ce7;width:16px;height:16px"
              />
              All products
            </label>
          </div>
          <button type="submit" disabled={checkinFormLoading}
            style="{btnPrimary};opacity:{checkinFormLoading ? 0.6 : 1}"
          >{checkinFormLoading ? 'Creating...' : 'Create'}</button>
        </form>
      </div>

      {#if checkinLoading}
        <p style="color:#aaa;text-align:center;padding:20px">Loading check-in lists...</p>
      {:else if checkinError}
        <p style="color:#e74c3c;text-align:center;padding:20px">{checkinError}</p>
      {:else if checkinLists.length === 0}
        <p style="color:#888;text-align:center;padding:20px">No check-in lists yet.</p>
      {:else}
        <div style={cardStyle}>
          <table style="width:100%;border-collapse:collapse">
            <thead>
              <tr style="text-align:left;border-bottom:1px solid #2a2a4a">
                <th style="padding:10px 12px;font-size:.85rem;color:#888;font-weight:600">Name</th>
                <th style="padding:10px 12px;font-size:.85rem;color:#888;font-weight:600">Products</th>
                <th style="padding:10px 12px;font-size:.85rem;color:#888;font-weight:600"></th>
              </tr>
            </thead>
            <tbody>
              {#each checkinLists as list (list.id)}
                <tr style="border-bottom:1px solid #2a2a4a">
                  <td style="padding:12px;color:#eee;font-size:.95rem">{list.name}</td>
                  <td style="padding:12px;color:#aaa;font-size:.85rem">{list.all_products ? 'All' : 'Selected'}</td>
                  <td style="padding:12px;text-align:right">
                    <a href="/admin/checkin?list={list.id}"
                      style="color:#7c5ce7;text-decoration:none;font-size:.9rem;font-weight:600"
                    >View Checkin &rarr;</a>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}

    {:else if tab === 'settings'}
      <div style={cardStyle}>
        <h2 style="font-size:1.2rem;color:#eee;margin:0 0 16px">Event Settings</h2>
        <form onsubmit={handleUpdateEvent} style="display:flex;flex-direction:column;gap:14px">
          {#if settingsError}
            <p style="color:#e74c3c;margin:0;font-size:.9rem">{settingsError}</p>
          {/if}
          <div style="display:grid;grid-template-columns:1fr 1fr;gap:12px">
            <label style={labelStyle}>
              Name *
              <input type="text" bind:value={settingsForm.name} required style={inputStyle} />
            </label>
            <label style={labelStyle}>
              Location
              <input type="text" bind:value={settingsForm.location} style={inputStyle} />
            </label>
          </div>
          <div style="display:grid;grid-template-columns:1fr 1fr;gap:12px">
            <label style={labelStyle}>
              Date From *
              <input type="datetime-local" bind:value={settingsForm.date_from} required style={inputStyle} />
            </label>
            <label style={labelStyle}>
              Timezone *
              <input type="text" bind:value={settingsForm.timezone} required style={inputStyle} />
            </label>
          </div>
          <div style="display:grid;grid-template-columns:1fr 1fr;gap:12px">
            <label style={labelStyle}>
              Presale Start
              <input type="datetime-local" bind:value={settingsForm.presale_start} style={inputStyle} />
            </label>
            <label style={labelStyle}>
              Presale End
              <input type="datetime-local" bind:value={settingsForm.presale_end} style={inputStyle} />
            </label>
          </div>
          <label style="display:flex;align-items:center;gap:8px;color:#ccc;font-size:.9rem;cursor:pointer;width:fit-content">
            <input
              type="checkbox"
              bind:checked={settingsForm.live}
              style="accent-color:#7c5ce7;width:18px;height:18px"
            />
            Event is live
          </label>
          <button type="submit" disabled={settingsLoading}
            style="{btnPrimary};opacity:{settingsLoading ? 0.6 : 1}"
          >{settingsLoading ? 'Saving...' : 'Save Settings'}</button>
        </form>
      </div>
    {/if}
  {/if}
</div>
