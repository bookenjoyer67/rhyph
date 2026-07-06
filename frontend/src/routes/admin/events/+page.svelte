<script lang="ts">
  import { listEvents, createEvent } from '$lib/api';
  import { goto } from '$app/navigation';

  interface Event {
    id: string;
    slug: string;
    name: string;
    location: string;
    date_from: string;
    date_to: string | null;
    timezone: string;
    live: boolean;
  }

  let events = $state<Event[]>([]);
  let loading = $state(true);
  let error = $state('');
  let showForm = $state(false);
  let formLoading = $state(false);
  let formError = $state('');

  let slug = $state('');
  let name = $state('');
  let dateFrom = $state('');
  let location = $state('');
  let timezone = $state('UTC');

  $effect(() => {
    loadEvents();
  });

  async function loadEvents() {
    loading = true;
    error = '';
    try {
      events = await listEvents('default');
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Failed to load events';
    } finally {
      loading = false;
    }
  }

  async function handleCreate(e: Event) {
    e.preventDefault();
    formLoading = true;
    formError = '';
    try {
      await createEvent('default', {
        slug,
        name,
        date_from: new Date(dateFrom).toISOString(),
        location,
        timezone,
      });
      slug = '';
      name = '';
      dateFrom = '';
      location = '';
      timezone = 'UTC';
      showForm = false;
      await loadEvents();
    } catch (err: unknown) {
      formError = err instanceof Error ? err.message : 'Failed to create event';
    } finally {
      formLoading = false;
    }
  }
</script>

<div style="padding:32px;max-width:1000px;margin:0 auto">
  <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:24px">
    <h1 style="font-size:1.6rem;color:#eee;margin:0">Events</h1>
    <button
      onclick={() => showForm = true}
      style="background:#7c5ce7;color:#fff;border:none;padding:10px 24px;border-radius:6px;cursor:pointer;font-size:.95rem"
      onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = '#6a4fd4'}
      onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = '#7c5ce7'}
    >
      Create Event
    </button>
  </div>

  {#if showForm}
    <div style="background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:24px;margin-bottom:24px">
      <h2 style="font-size:1.2rem;color:#eee;margin:0 0 16px">Create Event</h2>
      <form onsubmit={handleCreate} style="display:flex;flex-direction:column;gap:14px">
        {#if formError}
          <p style="color:#e74c3c;margin:0;font-size:.9rem">{formError}</p>
        {/if}
        <div style="display:grid;grid-template-columns:1fr 2fr;gap:12px">
          <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">
            Slug *
            <input type="text" bind:value={slug} required
              style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none" />
          </label>
          <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">
            Name *
            <input type="text" bind:value={name} required
              style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none" />
          </label>
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr 1fr;gap:12px">
          <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">
            Date From *
            <input type="datetime-local" bind:value={dateFrom} required
              style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none" />
          </label>
          <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">
            Location
            <input type="text" bind:value={location}
              style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none" />
          </label>
          <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">
            Timezone *
            <input type="text" bind:value={timezone} required
              style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none" />
          </label>
        </div>
        <div style="display:flex;gap:12px">
          <button type="submit" disabled={formLoading}
            style="background:#7c5ce7;color:#fff;border:none;padding:10px 24px;border-radius:6px;cursor:pointer;font-size:.95rem;opacity:{formLoading ? 0.6 : 1}"
          >{formLoading ? 'Creating...' : 'Create'}</button>
          <button type="button" onclick={() => showForm = false}
            style="background:transparent;color:#aaa;border:1px solid #2a2a4a;padding:10px 24px;border-radius:6px;cursor:pointer;font-size:.95rem"
          >Cancel</button>
        </div>
      </form>
    </div>
  {/if}

  {#if loading}
    <p style="color:#aaa;text-align:center;padding:40px">Loading...</p>
  {:else if error}
    <p style="color:#e74c3c;text-align:center;padding:40px">{error}</p>
  {:else if events.length === 0}
    <p style="color:#aaa;text-align:center;padding:40px">No events yet. Create one to get started.</p>
  {:else}
    <div style="display:flex;flex-direction:column;gap:12px">
      {#each events as event (event.id)}
        <div style="background:#1a1a2e;border:1px solid #2a2a4a;border-radius:10px;padding:20px;display:flex;align-items:center;justify-content:space-between">
          <div>
            <h3 style="font-size:1.1rem;color:#eee;margin:0 0 4px">{event.name}</h3>
            <p style="font-size:.85rem;color:#888;margin:0">
              {new Date(event.date_from).toLocaleDateString()} {event.location ? `· ${event.location}` : ''}
            </p>
          </div>
          <span style="display:inline-block;padding:4px 14px;background:{event.live ? '#1a472a' : '#2a2a4a'};color:{event.live ? '#4ade80' : '#888'};border-radius:20px;font-size:.8rem;font-weight:600">
            {event.live ? 'Live' : 'Draft'}
          </span>
        </div>
      {/each}
    </div>
  {/if}
</div>
