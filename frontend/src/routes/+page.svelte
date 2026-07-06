<script lang="ts">
  import { getOrganizer, listEvents } from '$lib/api';
  import { goto } from '$app/navigation';

  interface EventData {
    id: string;
    slug: string;
    name: string;
    description: string;
    location: string;
    date_from: string;
    date_to: string | null;
    timezone: string;
    live: boolean;
  }

  interface VenueConfig {
    slug: string;
    name: string;
    theme: Record<string, unknown>;
    custom_domain: string | null;
  }

  let venue = $state<VenueConfig | null>(null);
  let events = $state<EventData[]>([]);
  let loading = $state(true);
  let error = $state('');

  $effect(() => {
    loadPage();
  });

  async function loadPage() {
    loading = true;
    try {
      const [org, evts] = await Promise.all([
        getOrganizer('default'),
        listEvents('default'),
      ]);
      venue = org;
      events = (evts as EventData[]).filter((e: EventData) => e.live);
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Failed to load';
    } finally {
      loading = false;
    }
  }

  function formatDate(iso: string): string {
    const d = new Date(iso);
    return d.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric', hour: 'numeric', minute: '2-digit' });
  }

  function stripHtml(html: string): string {
    if (!html) return '';
    return html.replace(/<[^>]*>/g, '').substring(0, 120) + '...';
  }

  const primary = $derived((venue?.theme?.primary_color as string) || '#FF1493');
  const accent = $derived((venue?.theme?.accent_color as string) || '#00E676');
  const venueName = $derived(venue?.name || 'The Neon Cathedral');
  const description = $derived((venue?.theme?.venue_description as string) || '');
  const address = $derived((venue?.theme?.venue_address as string) || '');
  const capacity = $derived((venue?.theme?.venue_capacity as string) || '');
</script>

{#if loading}
  <div style="display:flex;align-items:center;justify-content:center;min-height:calc(100vh - 61px);background:var(--rhyph-bg, #0a0a0f);color:#aaa">
    <p style="font-size:1.3rem;letter-spacing:2px;text-transform:uppercase">Loading...</p>
  </div>
{:else if error}
  <div style="display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:calc(100vh - 61px);background:var(--rhyph-bg, #0a0a0f);gap:16px">
    <p style="color:#e74c3c;font-size:1.2rem">{error}</p>
  </div>
{:else}
  <div style="min-height:calc(100vh - 61px);background:var(--rhyph-bg, #0a0a0f);color:#eee">

    <!-- ===== HERO ===== -->
    <div style="padding:80px 24px 60px;text-align:center;max-width:800px;margin:0 auto">
      <div style="font-size:.85rem;text-transform:uppercase;letter-spacing:4px;color:{primary};margin-bottom:16px;font-weight:600">Live Music · Warehouse District</div>
      <h1 style="font-size:3.5rem;font-weight:800;color:#fff;margin:0 0 8px;letter-spacing:-1px;line-height:1.1">
        {venueName}
      </h1>
      <p style="font-size:1.2rem;color:#888;margin:0 auto;max-width:550px;line-height:1.6">
        {description}
      </p>

      {#if address}
        <p style="font-size:.95rem;color:#666;margin:20px 0 0">
          {address}{capacity ? ' · ' + capacity + ' capacity' : ''}
        </p>
      {/if}
    </div>

    <!-- ===== UPCOMING EVENTS ===== -->
    <div style="max-width:900px;margin:0 auto;padding:0 24px 80px">
      <div style="display:flex;align-items:center;gap:12px;margin-bottom:32px">
        <div style="width:4px;height:28px;background:{primary};border-radius:2px"></div>
        <h2 style="font-size:1.6rem;font-weight:800;color:#fff;margin:0">Upcoming</h2>
      </div>

      {#if events.length === 0}
        <div style="background:var(--rhyph-card-bg, #12121f);border:1px solid var(--rhyph-card-border, #2a1a3a);border-radius:12px;padding:48px 24px;text-align:center">
          <p style="color:#666;font-size:1.1rem;margin:0">No upcoming events yet. Check back soon.</p>
        </div>
      {:else}
        <div style="display:flex;flex-direction:column;gap:16px">
          {#each events as event (event.id)}
            <a
              href="/events/default/{event.slug}"
              style="display:block;text-decoration:none;background:var(--rhyph-card-bg, #12121f);border:1px solid var(--rhyph-card-border, #2a1a3a);border-radius:14px;padding:28px 32px;transition:border-color .2s,transform .2s"
              onmouseenter={(e) => {
                (e.currentTarget as HTMLElement).style.borderColor = primary;
                (e.currentTarget as HTMLElement).style.transform = 'translateY(-2px)';
              }}
              onmouseleave={(e) => {
                (e.currentTarget as HTMLElement).style.borderColor = 'var(--rhyph-card-border, #2a1a3a)';
                (e.currentTarget as HTMLElement).style.transform = '';
              }}
            >
              <div style="display:flex;align-items:flex-start;justify-content:space-between;flex-wrap:wrap;gap:16px">
                <div style="flex:1;min-width:200px">
                  <div style="font-size:.8rem;text-transform:uppercase;letter-spacing:2px;color:{primary};margin-bottom:6px;font-weight:600">{formatDate(event.date_from)}</div>
                  <h3 style="font-size:1.35rem;font-weight:800;color:#fff;margin:0 0 8px;letter-spacing:-0.3px">{event.name}</h3>
                  {#if event.description}
                    <p style="font-size:.9rem;color:#888;margin:0;line-height:1.5;max-width:500px">{@html stripHtml(event.description)}</p>
                  {/if}
                </div>

                <div style="display:flex;align-items:center;gap:8px;flex-shrink:0">
                  <span style="color:{primary};font-size:.9rem;font-weight:700">Tickets →</span>
                </div>
              </div>
            </a>
          {/each}
        </div>
      {/if}
    </div>

    <!-- ===== INFO SECTION ===== -->
    <div style="border-top:1px solid var(--rhyph-card-border, #2a1a3a);padding:60px 24px">
      <div style="max-width:900px;margin:0 auto;display:grid;grid-template-columns:repeat(auto-fit, minmax(220px, 1fr));gap:32px">
        <div>
          <div style="font-size:.75rem;text-transform:uppercase;letter-spacing:3px;color:{primary};margin-bottom:8px;font-weight:700">Location</div>
          <p style="color:#aaa;font-size:.95rem;margin:0;line-height:1.5">{address || '2147 St. Claude Ave, New Orleans, LA'}</p>
        </div>
        <div>
          <div style="font-size:.75rem;text-transform:uppercase;letter-spacing:3px;color:{primary};margin-bottom:8px;font-weight:700">Capacity</div>
          <p style="color:#aaa;font-size:.95rem;margin:0">{capacity || '800'} standing</p>
        </div>
        <div>
          <div style="font-size:.75rem;text-transform:uppercase;letter-spacing:3px;color:{primary};margin-bottom:8px;font-weight:700">Box Office</div>
          <p style="color:#aaa;font-size:.95rem;margin:0;line-height:1.5">Open 1hr before doors<br />No phone orders</p>
        </div>
        <div>
          <div style="font-size:.75rem;text-transform:uppercase;letter-spacing:3px;color:{primary};margin-bottom:8px;font-weight:700">Powered by</div>
          <p style="color:#aaa;font-size:.95rem;margin:0">Rhyph — self-hosted ticketing</p>
        </div>
      </div>
    </div>

  </div>
{/if}
