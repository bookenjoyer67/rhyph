<script lang="ts">
  import { goto } from '$app/navigation';
</script>

<div style="display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:calc(100vh - 61px);background:var(--rhyph-bg, #111);padding:48px 24px;text-align:center">
  <h1 style="font-size:3.5rem;font-weight:800;color:#fff;margin:0 0 16px;letter-spacing:-1px">Rhyph</h1>
  <p style="font-size:1.3rem;color:#888;margin:0 0 8px;max-width:500px">
    Self-hosted ticketing for venues and bands.
  </p>
  <p style="font-size:1rem;color:#666;margin:0 0 40px;max-width:450px">
    No third party. No per-ticket fees. Your events, your data.
  </p>

  <div style="display:flex;gap:16px;flex-wrap:wrap;justify-content:center">
    <button
      onclick={() => goto('/login')}
      style="background:var(--rhyph-primary, #7c5ce7);color:#fff;border:none;padding:14px 36px;border-radius:10px;cursor:pointer;font-size:1.1rem;font-weight:700"
      onmouseenter={(e) => (e.currentTarget as HTMLElement).style.opacity = '0.85'}
      onmouseleave={(e) => (e.currentTarget as HTMLElement).style.opacity = '1'}
    >Admin Login</button>
    <button
      onclick={() => goto('/scan')}
      style="background:transparent;color:#7c5ce7;border:2px solid #7c5ce7;padding:14px 36px;border-radius:10px;cursor:pointer;font-size:1.1rem;font-weight:700"
      onmouseenter={(e) => { (e.currentTarget as HTMLElement).style.background = '#7c5ce722' }}
      onmouseleave={(e) => { (e.currentTarget as HTMLElement).style.background = 'transparent' }}
    >Scanner</button>
  </div>
</div>
