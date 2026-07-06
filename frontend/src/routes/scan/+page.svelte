<script lang="ts">
  import { setToken, listCheckinLists, getCheckinStats } from '$lib/api';

  /// API key
  const storedKey = localStorage.getItem('rhyph_scanner_key') || '';
  let apiKey = $state(storedKey);
  let apiKeyInput = $state('');

  /// Event + checkin lists
  const storedEvent = localStorage.getItem('rhyph_scanner_event') || '';
  let eventSlug = $state(storedEvent);
  let checkinLists = $state<{ id: string; name: string }[]>([]);
  let selectedListId = $state('');
  let listsLoading = $state(false);
  let listsError = $state('');

  /// Scan input
  let secret = $state('');
  let scanning = $state(false);
  let inputEl = $state<HTMLInputElement | null>(null);

  /// Result overlay
  interface ScanResult {
    success: boolean;
    message: string;
    name?: string;
    ticketType?: string;
  }
  let result = $state<ScanResult | null>(null);
  let resultTimer: ReturnType<typeof setTimeout> | null = null;

  /// Stats counter
  let scannedToday = $state(0);
  let statsInterval: ReturnType<typeof setInterval> | null = null;

  /// Camera
  let cameraActive = $state(false);
  let cameraError = $state('');
  let cameraStream: MediaStream | null = null;
  let videoEl = $state<HTMLVideoElement | null>(null);
  let canvasEl = $state<HTMLCanvasElement | null>(null);
  let animFrame = 0;
  let barcodeDetector: BarcodeDetector | null = null;
  const supportsBarcode = typeof window !== 'undefined' && 'BarcodeDetector' in window;

  /// Init
  $effect(() => {
    if (apiKey) setToken(apiKey);
  });

  $effect(() => {
    if (apiKey && eventSlug) {
      fetchLists();
    }
  });

  $effect(() => {
    if (!apiKey || !selectedListId) return;
    fetchStats();
    statsInterval = setInterval(fetchStats, 30_000);
    return () => {
      if (statsInterval) clearInterval(statsInterval);
    };
  });

  $effect(() => {
    return () => {
      stopCamera();
      if (statsInterval) clearInterval(statsInterval);
    };
  });

  /// --- Helpers ----------------------------------------------------------------

  function handleConnect() {
    const key = apiKeyInput.trim();
    if (!key) return;
    apiKey = key;
    localStorage.setItem('rhyph_scanner_key', apiKey);
    setToken(apiKey);
  }

  function handleDisconnect() {
    localStorage.removeItem('rhyph_scanner_key');
    apiKey = '';
    apiKeyInput = '';
    setToken(null);
    selectedListId = '';
    checkinLists = [];
    scannedToday = 0;
    stopCamera();
    if (statsInterval) clearInterval(statsInterval);
  }

  async function fetchLists() {
    listsLoading = true;
    listsError = '';
    try {
      checkinLists = await listCheckinLists('default', eventSlug);
    } catch (err: unknown) {
      listsError = err instanceof Error ? err.message : 'Failed to load lists';
      checkinLists = [];
    } finally {
      listsLoading = false;
    }
  }

  async function fetchStats() {
    if (!selectedListId) return;
    try {
      const s = await getCheckinStats(selectedListId);
      scannedToday = s.total_scans ?? 0;
    } catch { /* silent */ }
  }

  function onEventChange() {
    const val = eventSlug.trim();
    localStorage.setItem('rhyph_scanner_event', val);
    selectedListId = '';
    checkinLists = [];
    if (apiKey && val) fetchLists();
  }

  /// --- Sound ---------------------------------------------------------------

  function playBeep() {
    try {
      const ctx = new AudioContext();
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.connect(gain);
      gain.connect(ctx.destination);
      osc.type = 'square';
      osc.frequency.value = 880;
      gain.gain.setValueAtTime(0.06, ctx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + 0.18);
      osc.start(ctx.currentTime);
      osc.stop(ctx.currentTime + 0.18);
    } catch { /* audio not available */ }
  }

  function playBuzz() {
    try {
      const ctx = new AudioContext();
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.connect(gain);
      gain.connect(ctx.destination);
      osc.type = 'sawtooth';
      osc.frequency.value = 180;
      gain.gain.setValueAtTime(0.08, ctx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + 0.35);
      osc.start(ctx.currentTime);
      osc.stop(ctx.currentTime + 0.35);
    } catch { /* audio not available */ }
  }

  /// --- Result overlay ----------------------------------------------------

  function showResult(
    success: boolean,
    data: { error?: string; attendee_name?: string; ticket_type?: string },
  ) {
    if (resultTimer) clearTimeout(resultTimer);
    result = {
      success,
      message: success ? 'Checked in!' : (data.error || 'Scan failed'),
      name: data.attendee_name,
      ticketType: data.ticket_type,
    };
    resultTimer = setTimeout(() => {
      result = null;
    }, 3000);
  }

  /// --- Scan ---------------------------------------------------------------

  async function doScan(scannedSecret: string) {
    const s = scannedSecret.trim();
    if (!s || !selectedListId || scanning) return;
    scanning = true;
    try {
      const nonce = crypto.randomUUID();
      const res = await fetch(`/api/v1/checkin/lists/${selectedListId}/scan`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${apiKey}`,
        },
        body: JSON.stringify({ secret: s, nonce }),
      });
      const data = await res.json();
      if (res.ok) {
        playBeep();
        showResult(true, data);
        fetchStats();
      } else {
        playBuzz();
        showResult(false, data);
      }
    } catch (err: unknown) {
      playBuzz();
      showResult(false, { error: err instanceof Error ? err.message : 'Network error' });
    } finally {
      scanning = false;
      secret = '';
      inputEl?.focus();
    }
  }

  function onSecretSubmit(e: Event) {
    e.preventDefault();
    doScan(secret);
  }

  /// --- Camera QR scanner --------------------------------------------------

  async function startCamera() {
    cameraError = '';
    if (!supportsBarcode) {
      cameraError = 'Barcode detection not supported in this browser — use Chrome on Android.';
      return;
    }
    try {
      cameraStream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: 'environment', width: { ideal: 640 }, height: { ideal: 480 } },
        audio: false,
      });
      cameraActive = true;
      if (!barcodeDetector) {
        barcodeDetector = new BarcodeDetector({ formats: ['qr_code'] });
      }
      await new Promise((r) => setTimeout(r, 80));
      if (videoEl && cameraStream) {
        videoEl.srcObject = cameraStream;
        await videoEl.play();
        const vw = videoEl.videoWidth || 640;
        const vh = videoEl.videoHeight || 480;
        if (canvasEl) {
          canvasEl.width = vw;
          canvasEl.height = vh;
        }
        animFrame = requestAnimationFrame(scanFrame);
      }
    } catch (err: unknown) {
      cameraError = err instanceof Error ? err.message : 'Camera access denied';
      cameraActive = false;
      cameraStream = null;
    }
  }

  function stopCamera() {
    if (animFrame) {
      cancelAnimationFrame(animFrame);
      animFrame = 0;
    }
    if (cameraStream) {
      cameraStream.getTracks().forEach((t) => t.stop());
      cameraStream = null;
    }
    if (videoEl) {
      videoEl.srcObject = null;
    }
    cameraActive = false;
  }

  function scanFrame() {
    if (!cameraActive || !videoEl || !canvasEl) return;
    const ctx = canvasEl.getContext('2d', { willReadFrequently: true });
    if (!ctx) return;
    ctx.drawImage(videoEl, 0, 0, canvasEl.width, canvasEl.height);

    if (barcodeDetector) {
      createImageBitmap(canvasEl)
        .then((bitmap) => barcodeDetector!.detect(bitmap))
        .then((barcodes: { rawValue: string }[]) => {
          if (barcodes && barcodes.length > 0) {
            stopCamera();
            doScan(barcodes[0].rawValue);
          }
        })
        .catch(() => { /* skip frame */ });
    }
    animFrame = requestAnimationFrame(scanFrame);
  }
</script>

<!-- ====== API KEY CONNECT ====== -->
{#if !apiKey}
  <div
    style="display:flex;align-items:center;justify-content:center;min-height:100vh;min-height:100dvh;background:#111;padding:24px"
  >
    <form
      onsubmit={(e) => { e.preventDefault(); handleConnect(); }}
      style="display:flex;flex-direction:column;gap:16px;width:100%;max-width:400px;padding:32px 24px;background:#1a1a2e;border-radius:16px;border:1px solid #2a2a4a"
    >
      <h1 style="font-size:1.6rem;font-weight:700;color:#7c5ce7;text-align:center;margin:0">Rhyph Scanner</h1>
      <p style="color:#888;text-align:center;margin:0;font-size:.9rem">Enter your scanner API key to connect</p>
      <input
        type="password"
        bind:value={apiKeyInput}
        placeholder="Scanner API key"
        autocomplete="off"
        style="padding:14px 16px;border:1px solid #2a2a4a;border-radius:10px;background:#0d0d1a;color:#eee;font-size:1.05rem;outline:none;min-height:48px"
        onkeydown={(e) => { if (e.key === 'Enter') handleConnect(); }}
      />
      <button
        type="submit"
        disabled={!apiKeyInput.trim()}
        style="background:#7c5ce7;color:#fff;border:none;padding:14px;border-radius:10px;cursor:pointer;font-size:1.1rem;font-weight:600;min-height:48px;opacity:{apiKeyInput.trim() ? '1' : '0.4'}"
      >Connect</button>
    </form>
  </div>
{:else if !eventSlug || !selectedListId}
  <!-- ====== SETUP: event + list selection ====== -->
  <div
    style="display:flex;flex-direction:column;min-height:100vh;min-height:100dvh;background:#111;padding:16px;gap:16px"
  >
    <div style="display:flex;align-items:center;justify-content:space-between">
      <h1 style="font-size:1.2rem;font-weight:700;color:#7c5ce7;margin:0">Rhyph Scanner</h1>
      <button
        onclick={handleDisconnect}
        style="background:transparent;color:#e74c3c;border:1px solid #e74c3c;padding:8px 16px;border-radius:8px;font-size:.85rem;cursor:pointer;min-height:48px"
      >Disconnect</button>
    </div>

    <div style="display:flex;flex-direction:column;gap:12px;flex:1">
      <label style="display:flex;flex-direction:column;gap:6px;color:#aaa;font-size:.85rem">
        Event slug
        <input
          type="text"
          bind:value={eventSlug}
          placeholder="e.g. summer-fest-2026"
          style="padding:12px 16px;border:1px solid #2a2a4a;border-radius:10px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none;min-height:48px"
          onchange={onEventChange}
          onblur={onEventChange}
        />
      </label>

      <label style="display:flex;flex-direction:column;gap:6px;color:#aaa;font-size:.85rem">
        Check-in list
        <select
          bind:value={selectedListId}
          disabled={listsLoading || checkinLists.length === 0}
          style="padding:12px 16px;border:1px solid #2a2a4a;border-radius:10px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none;min-height:48px;appearance:none"
          onchange={() => { if (selectedListId) fetchStats(); }}
        >
          <option value="">{listsLoading ? 'Loading...' : listsError || 'Select a list'}</option>
          {#each checkinLists as list (list.id)}
            <option value={list.id}>{list.name}</option>
          {/each}
        </select>
      </label>

      {#if listsError}
        <p style="color:#e74c3c;font-size:.85rem;margin:0">{listsError}</p>
      {/if}
    </div>
  </div>
{:else if cameraActive}
  <!-- ====== CAMERA VIEW ====== -->
  <div
    style="position:fixed;inset:0;background:#000;z-index:100;display:flex;flex-direction:column"
  >
    <video
      bind:this={videoEl}
      playsinline
      autoplay
      muted
      style="width:100%;flex:1;object-fit:cover;display:block"
    ></video>
    <canvas bind:this={canvasEl} style="display:none"></canvas>
    <div style="padding:12px 16px;display:flex;align-items:center;justify-content:space-between;gap:12px;background:rgba(0,0,0,0.85)">
      <span style="color:#aaa;font-size:.85rem">Point camera at QR code</span>
      <button
        onclick={stopCamera}
        style="background:#e74c3c;color:#fff;border:none;padding:10px 20px;border-radius:8px;font-size:.95rem;cursor:pointer;min-height:48px"
      >Close</button>
    </div>
  </div>
{:else}
  <!-- ====== SCANNER UI ====== -->
  <div
    style="display:flex;flex-direction:column;min-height:100vh;min-height:100dvh;background:#111"
  >
    <!-- Header bar -->
    <div
      style="display:flex;align-items:center;justify-content:space-between;padding:12px 16px;background:#1a1a2e;border-bottom:1px solid #2a2a4a;gap:8px"
    >
      <div style="display:flex;align-items:center;gap:8px;min-width:0;flex:1">
        <span style="color:#7c5ce7;font-weight:700;font-size:.95rem;white-space:nowrap">Rhyph</span>
        <select
          bind:value={selectedListId}
          style="padding:6px 8px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;font-size:.8rem;outline:none;max-width:160px;min-height:36px;appearance:none"
          onchange={() => { if (selectedListId) fetchStats(); }}
        >
          {#each checkinLists as list (list.id)}
            <option value={list.id}>{list.name}</option>
          {/each}
        </select>
      </div>
      <button
        onclick={handleDisconnect}
        style="background:transparent;color:#e74c3c;border:1px solid #e74c3c;padding:6px 12px;border-radius:6px;font-size:.8rem;cursor:pointer;min-height:36px;white-space:nowrap"
      >Disconnect</button>
    </div>

    <!-- Scan input area -->
    <div
      style="flex:1;display:flex;flex-direction:column;align-items:center;justify-content:center;padding:24px;gap:24px"
    >
      <form
        onsubmit={onSecretSubmit}
        style="width:100%;max-width:440px;display:flex;flex-direction:column;align-items:center;gap:16px"
      >
        <span style="color:#888;font-size:.85rem;text-align:center">
          Scan ticket or type secret below
        </span>
        <input
          bind:this={inputEl}
          bind:value={secret}
          type="text"
          inputmode="none"
          autocomplete="off"
          placeholder={scanning ? 'Scanning...' : 'Ticket secret'}
          disabled={scanning}
          style="width:100%;padding:18px;border:2px solid #2a2a4a;border-radius:14px;background:#0d0d1a;color:#eee;font-size:1.4rem;text-align:center;outline:none;min-height:64px;letter-spacing:.05em;text-transform:uppercase;opacity:{scanning ? '0.5' : '1'}"
        />
      </form>

      <button
        onclick={startCamera}
        disabled={scanning}
        style="display:flex;align-items:center;justify-content:center;gap:8px;background:#1a1a2e;color:#7c5ce7;border:2px solid #7c5ce7;padding:14px 28px;border-radius:14px;font-size:1.05rem;cursor:pointer;min-height:56px;width:100%;max-width:440px;opacity:{scanning ? '0.5' : '1'}"
      >
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="7" height="7" rx="1" />
          <rect x="14" y="3" width="7" height="7" rx="1" />
          <rect x="3" y="14" width="7" height="7" rx="1" />
          <rect x="14" y="14" width="7" height="7" rx="1" />
          <line x1="8.5" y1="8.5" x2="15.5" y2="15.5" />
        </svg>
        Scan QR Code
      </button>

      {#if !supportsBarcode}
        <p style="color:#666;font-size:.75rem;text-align:center;margin:0;max-width:400px">
          Camera scanner requires Chrome on Android. Use manual entry or a barcode scanner gun for now.
        </p>
      {/if}
    </div>

    <!-- Stats footer -->
    <div
      style="padding:14px 16px;background:#1a1a2e;border-top:1px solid #2a2a4a;text-align:center"
    >
      <span style="color:#888;font-size:.85rem">Scanned today: </span>
      <span style="color:#4ade80;font-size:1.1rem;font-weight:700">{scannedToday}</span>
    </div>
  </div>
{/if}

<!-- ====== RESULT OVERLAY ====== -->
{#if result}
  <div
    role="button"
    tabindex="0"
    style="position:fixed;inset:0;z-index:200;display:flex;align-items:center;justify-content:center;background:rgba(0,0,0,0.75);padding:32px"
    onclick={() => { result = null; if (resultTimer) clearTimeout(resultTimer); }}
    onkeydown={(e: KeyboardEvent) => { if (e.key === 'Enter' || e.key === 'Escape') { result = null; if (resultTimer) clearTimeout(resultTimer); } }}
  >
    <div
      role="button"
      tabindex="0"
      style="display:flex;flex-direction:column;align-items:center;gap:12px;background:{result.success ? '#0a2e1a' : '#2e0a0a'};border:2px solid {result.success ? '#22c55e' : '#ef4444'};border-radius:20px;padding:40px 32px;max-width:340px;width:100%;text-align:center"
      onclick={(e: Event) => e.stopPropagation()}
      onkeydown={(e: KeyboardEvent) => { e.stopPropagation(); }}
    >
      {#if result.success}
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="20 6 9 17 4 12" />
        </svg>
        <p style="color:#4ade80;font-size:1.6rem;font-weight:700;margin:0">{result.message}</p>
        {#if result.name}
          <p style="color:#eee;font-size:1.2rem;font-weight:600;margin:0">{result.name}</p>
        {/if}
        {#if result.ticketType}
          <p style="color:#aaa;font-size:.95rem;margin:0">{result.ticketType}</p>
        {/if}
      {:else}
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="#ef4444" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <line x1="18" y1="6" x2="6" y2="18" />
          <line x1="6" y1="6" x2="18" y2="18" />
        </svg>
        <p style="color:#f87171;font-size:1.3rem;font-weight:700;margin:0">{result.message}</p>
      {/if}
    </div>
  </div>
{/if}

{#if cameraError && !cameraActive}
  <div
    style="position:fixed;bottom:24px;left:24px;right:24px;z-index:150;background:#2e0a0a;border:1px solid #ef4444;border-radius:12px;padding:14px 18px;color:#f87171;font-size:.9rem;text-align:center"
  >{cameraError}</div>
{/if}
