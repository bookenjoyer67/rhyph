<script lang="ts">
  import { listEvents, getOrganizer } from '$lib/api';

  let orgName = $state('THE NEON CATHEDRAL');
  let primary = $state('#FF1493');
  let accent = $state('#00E676');
  let events = $state<any[]>([]);
  let loading = $state(true);

  $effect(() => {
    Promise.all([
      getOrganizer('default'),
      listEvents('default'),
    ]).then(([org, evts]) => {
      orgName = org.name;
      primary = (org.theme as Record<string,string>).primary_color || '#FF1493';
      accent = (org.theme as Record<string,string>).accent_color || '#00E676';
      events = (evts as any[]).filter(e => e.live);
      loading = false;
    }).catch(() => loading = false);
  });

  function formatDate(iso: string): string {
    const d = new Date(iso);
    return d.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric', hour: 'numeric', minute: '2-digit' });
  }

  function stripHtml(html: string): string {
    return html?.replace(/<[^>]*>/g, '').substring(0, 140) + '...' || '';
  }
</script>

<div style="--nc-primary:{primary};--nc-accent:{accent}">

  <!-- HERO with particle canvas -->
  <section style="position:relative;min-height:90vh;display:flex;align-items:center;justify-content:center;overflow:hidden">
    <!-- Particle canvas -->
    <canvas id="hero-canvas" style="position:absolute;inset:0;z-index:0"></canvas>

    <!-- Gothic arch overlay -->
    <div style="position:absolute;inset:0;z-index:1;background:radial-gradient(ellipse at center, transparent 40%, #08080f 80%)"></div>

    <div style="position:relative;z-index:2;text-align:center;padding:0 24px;max-width:800px">
      <!-- Animated arch frame -->
      <div style="margin-bottom:40px;position:relative;display:inline-block">
        <svg viewBox="0 0 200 60" style="width:120px;height:auto;filter:drop-shadow(0 0 8px {primary})" class="neon-glow">
          <path d="M10,60 Q10,0 100,0 Q190,0 190,60" fill="none" stroke={primary} stroke-width="1.5"/>
          <path d="M30,60 Q30,15 100,15 Q170,15 170,60" fill="none" stroke={accent} stroke-width="0.5" opacity="0.4"/>
        </svg>
      </div>

      <div class="reveal">
        <div style="font-size:.75rem;text-transform:uppercase;letter-spacing:6px;color:{accent};margin-bottom:20px;font-weight:600">New Orleans · Warehouse District</div>
        <h1 style="font-family:'Cinzel',serif;font-size:clamp(3rem,8vw,6rem);font-weight:900;color:#fff;margin:0;line-height:1;text-shadow:0 0 60px {primary}">
          THE NEON<br/>CATHEDRAL
        </h1>
        <div style="width:80px;height:2px;background:linear-gradient(90deg,transparent,{primary},transparent);margin:24px auto"></div>
        <p style="font-size:1.1rem;color:#888;max-width:500px;margin:0 auto 40px;line-height:1.6">
          A converted 1920s cathedral in the Warehouse District. 
          800 capacity. Two bars. The stained glass is original — the neon crosses are not.
        </p>
        <div style="display:flex;gap:16px;justify-content:center;flex-wrap:wrap" class="reveal reveal-d3">
          <a href="#events" style="display:inline-block;padding:16px 36px;background:{primary};color:#08080f;text-decoration:none;font-weight:700;letter-spacing:2px;font-size:.9rem;text-transform:uppercase;transition:box-shadow .3s, transform .3s"
             onmouseenter={(e) => { (e.currentTarget as HTMLElement).style.boxShadow = `0 0 40px ${primary}`; (e.currentTarget as HTMLElement).style.transform = 'scale(1.05)'; }}
             onmouseleave={(e) => { (e.currentTarget as HTMLElement).style.boxShadow = 'none'; (e.currentTarget as HTMLElement).style.transform = 'scale(1)'; }}
          >View Events</a>
          <a href="#info" style="display:inline-block;padding:16px 36px;border:1.5px solid {accent};color:{accent};text-decoration:none;font-weight:700;letter-spacing:2px;font-size:.9rem;text-transform:uppercase;transition:background .3s"
             onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = `${accent}11`}
             onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = 'transparent'}
          >Venue Info</a>
        </div>
      </div>
    </div>

    <!-- Floating particles -->
    <div style="position:absolute;inset:0;z-index:0;pointer-events:none;overflow:hidden">
      {#each Array(15) as _, i}
        <div style="position:absolute;left:{Math.random()*100}%;width:2px;height:2px;background:{i%2?primary:accent};border-radius:50%;animation:floatUp {3+Math.random()*5}s ease-in infinite;animation-delay:{Math.random()*5}s;opacity:{0.2+Math.random()*0.4}"></div>
      {/each}
    </div>
  </section>

  <!-- Events section -->
  <section id="events" style="padding:80px 24px;max-width:1100px;margin:0 auto">
    <div style="text-align:center;margin-bottom:60px" class="reveal">
      <div style="font-size:.7rem;text-transform:uppercase;letter-spacing:5px;color:{accent};margin-bottom:12px;font-weight:600">Upcoming</div>
      <h2 style="font-family:'Cinzel',serif;font-size:2.5rem;font-weight:900;color:#fff;margin:0;letter-spacing:2px">RITUALS</h2>
      <div style="width:60px;height:1px;background:{primary};margin:20px auto"></div>
    </div>

    {#if loading}
      <div style="text-align:center;padding:80px 0">
        <div style="width:40px;height:40px;border:2px solid #1a1a2e;border-top-color:{primary};border-radius:50%;margin:0 auto;animation:spin 1s linear infinite"></div>
      </div>
    {:else if events.length === 0}
      <div style="text-align:center;padding:60px 0;color:#666">No upcoming events. Check back.</div>
    {:else}
      <div style="display:flex;flex-direction:column;gap:20px">
        {#each events as event, idx (event.id)}
          <a href="/events/{event.slug}" style="text-decoration:none;display:block" class="reveal reveal-d{idx+1}">
            <div style="display:grid;grid-template-columns:auto 1fr auto;gap:24px;align-items:center;padding:28px 32px;background:rgba(255,255,255,0.02);border:1px solid rgba(255,255,255,0.06);transition:border-color .3s,transform .3s,background .3s;position:relative;overflow:hidden"
                 onmouseenter={(e) => {
                   (e.currentTarget as HTMLElement).style.borderColor = primary;
                   (e.currentTarget as HTMLElement).style.transform = 'translateX(8px)';
                   (e.currentTarget as HTMLElement).style.background = 'rgba(255,255,255,0.04)';
                 }}
                 onmouseleave={(e) => {
                   (e.currentTarget as HTMLElement).style.borderColor = 'rgba(255,255,255,0.06)';
                   (e.currentTarget as HTMLElement).style.transform = 'translateX(0)';
                   (e.currentTarget as HTMLElement).style.background = 'rgba(255,255,255,0.02)';
                 }}
            >
              <!-- Date block -->
              <div style="text-align:center;min-width:80px">
                <div style="font-family:'Cinzel',serif;font-size:2rem;font-weight:900;color:{primary};line-height:1">
                  {new Date(event.date_from).getDate()}
                </div>
                <div style="font-size:.75rem;text-transform:uppercase;letter-spacing:2px;color:#888;margin-top:4px">
                  {new Date(event.date_from).toLocaleDateString('en-US', { month: 'short' })}
                </div>
              </div>

              <!-- Event info -->
              <div>
                <h3 style="font-family:'Cinzel',serif;font-size:1.4rem;font-weight:700;color:#fff;margin:0 0 6px;letter-spacing:1px">{event.name}</h3>
                <div style="display:flex;gap:20px;flex-wrap:wrap">
                  <span style="font-size:.85rem;color:#888">{formatDate(event.date_from)}</span>
                  <span style="font-size:.85rem;color:{accent}">{event.location}</span>
                </div>
                {#if event.description}
                  <p style="font-size:.85rem;color:#666;margin:8px 0 0;line-height:1.5">{@html stripHtml(event.description)}</p>
                {/if}
              </div>

              <!-- Arrow -->
              <div style="color:{primary};font-size:1.5rem;transition:transform .3s">→</div>
            </div>
          </a>
        {/each}
      </div>
    {/if}
  </section>

  <!-- Venue info -->
  <section id="info" style="padding:80px 24px;border-top:1px solid rgba(255,255,255,0.04)">
    <div style="max-width:900px;margin:0 auto;display:grid;grid-template-columns:repeat(auto-fit,minmax(250px,1fr));gap:40px" class="reveal reveal-d2">
      <div style="text-align:center;padding:40px 24px;background:rgba(255,255,255,0.02);border:1px solid rgba(255,255,255,0.04)">
        <div style="font-size:2rem;margin-bottom:12px">⛪</div>
        <h3 style="font-family:'Cinzel',serif;color:{primary};margin:0 0 8px;letter-spacing:1px">THE SPACE</h3>
        <p style="color:#888;font-size:.9rem;line-height:1.6;margin:0">Converted 1920s cathedral. 40ft vaulted ceilings. Original stained glass windows. 800 standing capacity across two levels.</p>
      </div>
      <div style="text-align:center;padding:40px 24px;background:rgba(255,255,255,0.02);border:1px solid rgba(255,255,255,0.04)">
        <div style="font-size:2rem;margin-bottom:12px">🍸</div>
        <h3 style="font-family:'Cinzel',serif;color:{accent};margin:0 0 8px;letter-spacing:1px">THE BARS</h3>
        <p style="color:#888;font-size:.9rem;line-height:1.6;margin:0">Two full bars. The Sanctuary Bar on the main floor. The Confessional upstairs — cocktail program, absinthe fountain.</p>
      </div>
      <div style="text-align:center;padding:40px 24px;background:rgba(255,255,255,0.02);border:1px solid rgba(255,255,255,0.04)">
        <div style="font-size:2rem;margin-bottom:12px">🎵</div>
        <h3 style="font-family:'Cinzel',serif;color:{primary};margin:0 0 8px;letter-spacing:1px">THE SOUND</h3>
        <p style="color:#888;font-size:.9rem;line-height:1.6;margin:0">Custom Funktion-One system tuned for the room. The natural reverb of the cathedral is part of the mix.</p>
      </div>
    </div>
  </section>
</div>

<style>
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
