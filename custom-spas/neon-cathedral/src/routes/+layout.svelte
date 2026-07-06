<script lang="ts">
  import { getOrganizer } from '$lib/api';
  import { page } from '$app/stores';

  let { children } = $props();

  let orgName = $state('THE NEON CATHEDRAL');
  let primary = $state('#FF1493');
  let accent = $state('#00E676');

  $effect(() => {
    getOrganizer('default').then(o => {
      orgName = o.name.toUpperCase();
      primary = (o.theme as Record<string,string>).primary_color || '#FF1493';
      accent = (o.theme as Record<string,string>).accent_color || '#00E676';
    }).catch(() => {});
  });

  const path = $derived($page.url.pathname);

  // Scroll state for header shrink
  let scrolled = $state(false);
  $effect(() => {
    const onScroll = () => scrolled = window.scrollY > 60;
    window.addEventListener('scroll', onScroll, { passive: true });
    return () => window.removeEventListener('scroll', onScroll);
  });
</script>

<svelte:head>
  <style>
    @keyframes neonPulse {
      0%, 100% { filter: drop-shadow(0 0 4px var(--nc-primary)) drop-shadow(0 0 8px var(--nc-primary)); }
      50% { filter: drop-shadow(0 0 8px var(--nc-primary)) drop-shadow(0 0 16px var(--nc-primary)) drop-shadow(0 0 24px var(--nc-primary)); }
    }
    @keyframes floatUp {
      0% { transform: translateY(100vh) scale(0); opacity: 0; }
      10% { opacity: 1; }
      90% { opacity: 0.6; }
      100% { transform: translateY(-10vh) scale(1.5); opacity: 0; }
    }
    @keyframes archReveal {
      0% { clip-path: inset(0 100% 0 0); }
      100% { clip-path: inset(0 0 0 0); }
    }
    @keyframes fadeUp {
      0% { opacity: 0; transform: translateY(30px); }
      100% { opacity: 1; transform: translateY(0); }
    }
    @keyframes stainedGlass {
      0%, 100% { background-position: 0% 50%; }
      50% { background-position: 100% 50%; }
    }
    .reveal { animation: fadeUp 0.8s ease-out both; }
    .reveal-d1 { animation-delay: 0.1s; }
    .reveal-d2 { animation-delay: 0.2s; }
    .reveal-d3 { animation-delay: 0.3s; }
    .reveal-d4 { animation-delay: 0.4s; }
    .neon-glow { animation: neonPulse 2s ease-in-out infinite; }
    .stained-bg { background: linear-gradient(135deg, var(--nc-primary), var(--nc-accent), #7c3aed, var(--nc-primary)); background-size: 300% 300%; animation: stainedGlass 6s ease infinite; }
    .arch-clip { clip-path: polygon(0% 0%, 100% 0%, 100% 85%, 70% 100%, 50% 90%, 30% 100%, 0% 85%); }
  </style>
</svelte:head>

<div style="--nc-primary:{primary};--nc-accent:{accent};min-height:100vh;background:#08080f;color:#eee">

  <!-- Header -->
  <header style="position:fixed;top:0;left:0;right:0;z-index:100;padding:{scrolled?'8px 32px':'16px 32px'};background:rgba(8,8,15,{scrolled?'0.95':'0.85'});backdrop-filter:blur(12px);border-bottom:1px solid rgba(255,255,255,0.06);transition:padding .3s,background .3s;display:flex;align-items:center;justify-content:space-between">
    <a href="/" style="text-decoration:none;display:flex;align-items:center;gap:12px">
      <div style="width:36px;height:36px;position:relative">
        <!-- Gothic rose window SVG -->
        <svg viewBox="0 0 36 36" fill="none" style="width:36px;height:36px;filter:drop-shadow(0 0 6px {primary})">
          <circle cx="18" cy="18" r="17" stroke={primary} stroke-width="1.5" fill="none"/>
          <circle cx="18" cy="18" r="10" stroke={primary} stroke-width="1" fill="none" opacity="0.6"/>
          <circle cx="18" cy="18" r="4" fill={primary} opacity="0.3"/>
          <line x1="18" y1="1" x2="18" y2="35" stroke={primary} stroke-width="0.5" opacity="0.4"/>
          <line x1="1" y1="18" x2="35" y2="18" stroke={primary} stroke-width="0.5" opacity="0.4"/>
          <line x1="6" y1="6" x2="30" y2="30" stroke={primary} stroke-width="0.5" opacity="0.3"/>
          <line x1="30" y1="6" x2="6" y2="30" stroke={primary} stroke-width="0.5" opacity="0.3"/>
        </svg>
      </div>
      <span style="font-family:'Cinzel',serif;font-size:{scrolled?'1rem':'1.2rem'};font-weight:900;color:#fff;letter-spacing:3px;transition:font-size .3s;text-shadow:0 0 20px {primary}">{orgName}</span>
    </a>

    <nav style="display:flex;align-items:center;gap:24px">
      <a href="/" style="color:{path==='/'?primary:'#888'};text-decoration:none;font-size:.85rem;font-weight:600;letter-spacing:1px;text-transform:uppercase;transition:color .2s">Home</a>
      <a href="/events/void-drifter" style="color:{path.startsWith('/events')?primary:'#888'};text-decoration:none;font-size:.85rem;font-weight:600;letter-spacing:1px;text-transform:uppercase;transition:color .2s">Events</a>
      <a href="/cart?org=default&event=void-drifter" style="color:{path==='/cart'?primary:'#888'};text-decoration:none;font-size:.85rem;font-weight:600;letter-spacing:1px;text-transform:uppercase;transition:color .2s">
        Cart
        <span style="display:inline-block;margin-left:6px;padding:2px 8px;border-radius:10px;background:{primary};color:#08080f;font-size:.75rem;font-weight:700" id="cart-count">0</span>
      </a>
    </nav>
  </header>

  <!-- Content -->
  <main style="padding-top:72px">
    {@render children()}
  </main>

  <!-- Footer -->
  <footer style="border-top:1px solid rgba(255,255,255,0.06);padding:48px 32px 32px;margin-top:80px">
    <div style="max-width:1100px;margin:0 auto;display:grid;grid-template-columns:repeat(auto-fit,minmax(200px,1fr));gap:40px">
      <div>
        <h4 style="font-family:'Cinzel',serif;color:{primary};margin:0 0 12px;letter-spacing:2px;font-size:.9rem">{orgName}</h4>
        <p style="color:#666;font-size:.85rem;line-height:1.6;margin:0">2147 St. Claude Ave<br/>New Orleans, LA<br/>800 capacity</p>
      </div>
      <div>
        <h4 style="font-family:'Cinzel',serif;color:{primary};margin:0 0 12px;letter-spacing:2px;font-size:.9rem">BOX OFFICE</h4>
        <p style="color:#666;font-size:.85rem;line-height:1.6;margin:0">Open 1hr before doors<br/>No phone orders<br/>All sales final</p>
      </div>
      <div>
        <h4 style="font-family:'Cinzel',serif;color:{primary};margin:0 0 12px;letter-spacing:2px;font-size:.9rem">POWERED BY</h4>
        <p style="color:#555;font-size:.85rem;margin:0">Rhyph</p>
      </div>
    </div>
    <div style="text-align:center;margin-top:40px;padding-top:24px;border-top:1px solid rgba(255,255,255,0.04)">
      <p style="color:#444;font-size:.75rem;margin:0">A converted 1920s cathedral. The stained glass is original — the neon crosses are not.</p>
    </div>
  </footer>
</div>
