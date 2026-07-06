<script lang="ts">
  import { getOrganizer, updateOrganizer, uploadImage, listImages, deleteImage, getSpaStatus, uploadSpa, deleteSpa, type ImageInfo, type SpaStatus } from '$lib/api';

  let slug = $state('default');
  let name = $state('');
  let loading = $state(true);
  let saving = $state(false);
  let error = $state('');
  let saved = $state(false);
  let activeTab = $state<'theme' | 'custom-spa' | 'domain'>('theme');

  // Theme state
  let primaryColor = $state('#FF1493');
  let accentColor = $state('#00E676');
  let bgColor = $state('#0a0a0f');
  let cardBg = $state('#12121f');
  let cardBorder = $state('#2a1a3a');
  let fontFamily = $state('');
  let logoUrl = $state('');
  let venueDesc = $state('');
  let venueAddr = $state('');
  let venueCap = $state('');
  let customCss = $state('');
  let advOpen = $state(false);

  // Image state
  let images = $state<ImageInfo[]>([]);
  let uploadMsg = $state('');
  let uploading = $state(false);
  let dragOver = $state(false);

  // SPA state
  let spaStatus = $state<SpaStatus | null>(null);
  let spaUploadMsg = $state('');
  let spaUploading = $state(false);

  // Preset themes
  const presets = [
    {
      id: 'neon-cathedral',
      name: 'Neon Cathedral',
      tag: 'Default',
      primary: '#FF1493', accent: '#00E676', bg: '#0a0a0f', cardBg: '#12121f', cardBorder: '#2a1a3a',
    },
    {
      id: 'brutalist',
      name: 'Brutalist',
      primary: '#FFFFFF', accent: '#FF4444', bg: '#000000', cardBg: '#0a0a0a', cardBorder: '#1a1a1a',
      font: 'monospace',
    },
    {
      id: 'mojave',
      name: 'Mojave',
      primary: '#FF6B35', accent: '#F7DC6F', bg: '#1a1410', cardBg: '#241e18', cardBorder: '#3a2a1a',
    },
    {
      id: 'gallery',
      name: 'Gallery',
      primary: '#1A1A1A', accent: '#4169E1', bg: '#FAFAFA', cardBg: '#FFFFFF', cardBorder: '#E5E5E5',
    },
    {
      id: 'festival',
      name: 'Festival',
      primary: '#FF00FF', accent: '#FFFF00', bg: '#0D001A', cardBg: '#1A0033', cardBorder: '#330066',
    },
  ];

  $effect(() => {
    loadAll();
  });

  async function loadAll() {
    loading = true;
    try {
      const org = await getOrganizer(slug);
      name = org.name;
      const t = org.theme as Record<string, string>;
      primaryColor = t.primary_color || '#FF1493';
      accentColor = t.accent_color || '#00E676';
      bgColor = t.bg_color || '#0a0a0f';
      cardBg = t.card_bg || '#12121f';
      cardBorder = t.card_border || '#2a1a3a';
      fontFamily = t.font_family || '';
      logoUrl = t.logo_url || '';
      venueDesc = t.venue_description || '';
      venueAddr = t.venue_address || '';
      venueCap = t.venue_capacity || '';
      customCss = t.custom_css || '';
    } catch (e) { error = 'Failed to load'; }
    try {
      images = await listImages(slug);
    } catch { images = []; }
    try {
      spaStatus = await getSpaStatus(slug);
    } catch { spaStatus = null; }
    loading = false;
  }

  function applyPreset(p: typeof presets[0]) {
    primaryColor = p.primary;
    accentColor = p.accent;
    bgColor = p.bg;
    cardBg = p.cardBg;
    cardBorder = p.cardBorder;
    const maybeFont = (p as Record<string,string>).font;
    if (maybeFont) fontFamily = maybeFont;
    else fontFamily = '';
  }

  async function handleSave() {
    saving = true; error = ''; saved = false;
    try {
      const theme: Record<string, string> = { primary_color: primaryColor, accent_color: accentColor, bg_color: bgColor, card_bg: cardBg, card_border: cardBorder };
      if (fontFamily) theme.font_family = fontFamily;
      if (logoUrl) theme.logo_url = logoUrl;
      if (venueDesc) theme.venue_description = venueDesc;
      if (venueAddr) theme.venue_address = venueAddr;
      if (venueCap) theme.venue_capacity = venueCap;
      if (customCss) theme.custom_css = customCss;
      await updateOrganizer(slug, { theme });
      saved = true;
      setTimeout(() => saved = false, 3000);
    } catch (e) { error = e instanceof Error ? e.message : 'Failed'; }
    finally { saving = false; }
  }

  async function handleUpload(e: Event) {
    const input = e.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;
    await doUpload(file);
    input.value = '';
  }

  async function handleDrop(e: DragEvent) {
    dragOver = false;
    const file = e.dataTransfer?.files?.[0];
    if (!file) return;
    await doUpload(file);
  }

  async function doUpload(file: File) {
    uploading = true; uploadMsg = '';
    try {
      const img = await uploadImage(slug, file);
      images = [img, ...images];
      if (!logoUrl) {
        logoUrl = img.url;
        await handleSave(); // auto-save logo
      }
      uploadMsg = 'Uploaded!';
      setTimeout(() => uploadMsg = '', 2000);
    } catch (e) { uploadMsg = e instanceof Error ? e.message : 'Upload failed'; }
    finally { uploading = false; }
  }

  async function handleDelete(imgId: string) {
    try {
      await deleteImage(slug, imgId);
      images = images.filter(i => i.id !== imgId);
    } catch { /* silent */ }
  }

  function selImg(url: string) {
    logoUrl = url;
  }

  async function handleSpaUpload(e: Event) {
    const input = e.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;
    spaUploading = true; spaUploadMsg = '';
    try {
      const result = await uploadSpa(slug, file);
      spaUploadMsg = `Uploaded ${result.file_count} files!`;
      spaStatus = { has_custom_spa: true, index_exists: true, spa_updated_at: new Date().toISOString() };
      setTimeout(() => spaUploadMsg = '', 3000);
    } catch (err) { spaUploadMsg = err instanceof Error ? err.message : 'Upload failed'; }
    finally { spaUploading = false; input.value = ''; }
  }

  async function handleSpaFolderUpload(e: Event) {
    const input = e.currentTarget as HTMLInputElement;
    const files = input.files;
    if (!files || files.length === 0) return;
    spaUploading = true; spaUploadMsg = '';
    try {
      const form = new FormData();
      for (let i = 0; i < files.length; i++) {
        form.append('file', files[i], files[i].webkitRelativePath || files[i].name);
      }
      const headers: Record<string, string> = {};
      const token = localStorage.getItem('rhyph_token');
      if (token) headers['Authorization'] = `Bearer ${token}`;
      const res = await fetch(`/api/v1/admin/organizers/${slug}/spa/folder`, {
        method: 'POST',
        headers,
        body: form,
      });
      if (!res.ok) {
        const body = await res.json().catch(() => ({ error: res.statusText }));
        throw new Error(body.error || `HTTP ${res.status}`);
      }
      const result = await res.json();
      spaUploadMsg = `Uploaded ${result.file_count} files!`;
      spaStatus = { has_custom_spa: true, index_exists: true, spa_updated_at: new Date().toISOString() };
      setTimeout(() => spaUploadMsg = '', 3000);
    } catch (err) { spaUploadMsg = err instanceof Error ? err.message : 'Upload failed'; }
    finally { spaUploading = false; input.value = ''; }
  }

  async function handleSpaDelete() {
    if (!confirm('Remove custom SPA? This will revert to the default themed page.')) return;
    try {
      await deleteSpa(slug);
      spaStatus = { has_custom_spa: false, index_exists: false, spa_updated_at: null };
      spaUploadMsg = 'Removed.';
      setTimeout(() => spaUploadMsg = '', 2000);
    } catch (err) { spaUploadMsg = err instanceof Error ? err.message : 'Delete failed'; }
  }

  const inputStyle = 'padding:10px 14px;border:1px solid #2a2a4a;border-radius:8px;background:#0d0d1a;color:#eee;font-size:.95rem;outline:none;width:100%;box-sizing:border-box';
  const labelStyle = 'display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem';
  const cardStyle = 'background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:24px';
  const tabStyle = (t: string) => `padding:10px 20px;border:none;border-radius:8px;cursor:pointer;font-size:.9rem;font-weight:600;transition:background .2s;background:${activeTab === t ? '#7c5ce7' : 'transparent'};color:${activeTab === t ? '#fff' : '#888'}`;
</script>

<div style="max-width:1000px;margin:0 auto;padding:40px 24px">
  <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:32px">
    <h1 style="font-size:1.8rem;font-weight:800;color:#fff;margin:0">Public Page</h1>
    <div style="display:flex;align-items:center;gap:12px">
      {#if saved}<span style="color:#4ade80;font-size:.9rem;font-weight:600">Saved!</span>{/if}
      {#if error}<span style="color:#e74c3c;font-size:.85rem">{error}</span>{/if}
      <button onclick={handleSave} disabled={saving}
        style="background:#7c5ce7;color:#fff;border:none;padding:10px 24px;border-radius:8px;cursor:pointer;font-size:.9rem;font-weight:600;opacity:{saving?0.6:1}"
      >{saving ? 'Saving...' : 'Save'}</button>
    </div>
  </div>

  <!-- Tabs -->
  <div style="display:flex;gap:8px;margin-bottom:32px">
    <button onclick={() => activeTab = 'theme'} style={tabStyle('theme')}>Theme</button>
    <button onclick={() => activeTab = 'custom-spa'} style={tabStyle('custom-spa')}>Custom SPA</button>
    <button onclick={() => activeTab = 'domain'} style={tabStyle('domain')}>Domain</button>
  </div>

  {#if loading}
    <p style="color:#aaa;padding:40px 0">Loading...</p>
  {:else if activeTab === 'theme'}
    <!-- ===== THEME TAB ===== -->
    <div style="display:grid;grid-template-columns:1fr 300px;gap:24px">
      <div style="display:flex;flex-direction:column;gap:24px">

        <!-- Logo uploader -->
        <div style={cardStyle}>
          <h3 style="font-size:1rem;font-weight:700;color:#fff;margin:0 0 16px">Logo</h3>
          <div style="display:flex;align-items:center;gap:16px;flex-wrap:wrap">
            {#if logoUrl}
              <img src={logoUrl} alt="Logo" style="width:64px;height:64px;object-fit:contain;border-radius:8px;background:#0d0d1a;border:1px solid #2a2a4a" />
            {:else}
              <div style="width:64px;height:64px;border-radius:8px;background:#0d0d1a;border:2px dashed #2a2a4a;display:flex;align-items:center;justify-content:center;color:#555">No logo</div>
            {/if}
            <div style="flex:1;min-width:200px">
              <div
                style="border:2px dashed {dragOver ? '#7c5ce7' : '#2a2a4a'};border-radius:10px;padding:20px;text-align:center;transition:border-color .2s;background:{dragOver?'rgba(124,92,231,.08)':'transparent'};cursor:pointer"
                ondragover={(e) => { e.preventDefault(); dragOver = true; }}
                ondragleave={() => dragOver = false}
                ondrop={handleDrop}
                onclick={() => (document.querySelector('.logo-upload-input') as HTMLInputElement)?.click()}
              >
                <input type="file" accept="image/png,image/jpeg,image/webp,image/svg+xml" onchange={handleUpload} class="logo-upload-input" style="display:none" />
                {#if uploading}
                  <p style="color:#aaa;margin:0;font-size:.9rem">Uploading...</p>
                {:else}
                  <p style="color:#aaa;margin:0;font-size:.9rem">
                    <span style="color:#7c5ce7;font-weight:600">Click to upload</span> or drag and drop
                  </p>
                  <p style="color:#555;margin:4px 0 0;font-size:.8rem">PNG, JPG, WebP, SVG — max 5MB</p>
                {/if}
              </div>
              {#if uploadMsg}
                <p style="color:{uploadMsg.startsWith('Uploaded')?'#4ade80':'#e74c3c'};font-size:.85rem;margin:8px 0 0">{uploadMsg}</p>
              {/if}
            </div>
          </div>

          {#if images.length > 0}
            <div style="margin-top:16px">
              <p style="color:#666;font-size:.8rem;margin:0 0 8px">Uploaded images — click to set as logo</p>
              <div style="display:flex;gap:8px;flex-wrap:wrap">
                {#each images as img (img.id)}
                  <div style="position:relative;cursor:pointer" onclick={() => selImg(img.url)}>
                    <img src={img.url} alt={img.original_name} style="width:56px;height:56px;object-fit:cover;border-radius:6px;border:2px solid {logoUrl===img.url?'#7c5ce7':'#2a2a4a'}" />
                    <button onclick={(e: Event) => { e.stopPropagation(); handleDelete(img.id); }}
                      style="position:absolute;top:-6px;right:-6px;width:20px;height:20px;border-radius:50%;background:#e74c3c;color:#fff;border:none;font-size:.7rem;cursor:pointer;display:flex;align-items:center;justify-content:center;line-height:1">&times;</button>
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        </div>

        <!-- Preset themes -->
        <div style={cardStyle}>
          <h3 style="font-size:1rem;font-weight:700;color:#fff;margin:0 0 16px">Preset Themes</h3>
          <div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(150px,1fr));gap:10px">
            {#each presets as p}
              <button
                onclick={() => applyPreset(p)}
                style="text-align:left;padding:14px;border:2px solid {primaryColor===p.primary&&accentColor===p.accent?'#7c5ce7':'#2a2a4a'};border-radius:10px;background:#0d0d1a;cursor:pointer;transition:border-color .15s"
              >
                <div style="display:flex;gap:6px;margin-bottom:10px">
                  <div style="width:20px;height:20px;border-radius:4px;background:{p.primary}"></div>
                  <div style="width:20px;height:20px;border-radius:4px;background:{p.accent}"></div>
                  <div style="width:20px;height:20px;border-radius:4px;background:{p.bg};border:1px solid #333"></div>
                </div>
                <p style="color:#fff;font-size:.85rem;font-weight:700;margin:0 0 2px">{p.name}</p>
                {#if p.tag}<span style="font-size:.7rem;color:#7c5ce7;font-weight:600">{p.tag}</span>{/if}
              </button>
            {/each}
          </div>
        </div>

        <!-- Advanced -->
        <div style={cardStyle}>
          <button onclick={() => advOpen = !advOpen}
            style="display:flex;align-items:center;justify-content:space-between;width:100%;background:none;border:none;color:#aaa;cursor:pointer;padding:0;font-size:.95rem">
            <span style="font-weight:700">Advanced</span>
            <span style="font-size:1.2rem;transition:transform .2s;transform:{advOpen?'rotate(180deg)':'rotate(0)'}">▾</span>
          </button>
          {#if advOpen}
            <div style="display:flex;flex-direction:column;gap:14px;margin-top:16px;padding-top:16px;border-top:1px solid #2a2a4a">
              <label style={labelStyle}>Primary Color <input type="text" bind:value={primaryColor} style={inputStyle} /></label>
              <label style={labelStyle}>Accent Color <input type="text" bind:value={accentColor} style={inputStyle} /></label>
              <label style={labelStyle}>Background <input type="text" bind:value={bgColor} style={inputStyle} /></label>
              <label style={labelStyle}>Card BG <input type="text" bind:value={cardBg} style={inputStyle} /></label>
              <label style={labelStyle}>Card Border <input type="text" bind:value={cardBorder} style={inputStyle} /></label>
              <label style={labelStyle}>Font <input type="text" bind:value={fontFamily} placeholder="system-ui, sans-serif" style={inputStyle} /></label>
              <label style={labelStyle}>Custom CSS
                <textarea bind:value={customCss} rows="4" style="padding:10px;border:1px solid #2a2a4a;border-radius:8px;background:#0d0d1a;color:#eee;font-size:.85rem;outline:none;width:100%;box-sizing:border-box;font-family:monospace;resize:vertical"></textarea>
              </label>
              <label style={labelStyle}>Venue Description <textarea bind:value={venueDesc} rows="2" style="padding:10px;border:1px solid #2a2a4a;border-radius:8px;background:#0d0d1a;color:#eee;font-size:.85rem;outline:none;width:100%;box-sizing:border-box;resize:vertical"></textarea></label>
              <label style={labelStyle}>Address <input type="text" bind:value={venueAddr} style={inputStyle} /></label>
              <label style={labelStyle}>Capacity <input type="text" bind:value={venueCap} style={inputStyle} /></label>
            </div>
          {/if}
        </div>
      </div>

      <!-- Live preview panel -->
      <div style="position:sticky;top:20px;display:flex;flex-direction:column;gap:12px">
        <h3 style="font-size:1rem;font-weight:700;color:#fff;margin:0">Preview</h3>
        <div style="border-radius:12px;overflow:hidden;border:1px solid #2a2a4a;background:{bgColor}">
          <!-- Header -->
          <div style="padding:10px 14px;background:{bgColor};border-bottom:1px solid {cardBorder};display:flex;align-items:center;gap:8px">
            {#if logoUrl}
              <img src={logoUrl} alt="" style="width:22px;height:22px;object-fit:contain;border-radius:3px" />
            {/if}
            <span style="font-size:.8rem;font-weight:700;color:{primaryColor}">{name || 'Venue'}</span>
            <span style="margin-left:auto;font-size:.68rem;padding:3px 8px;border-radius:4px;background:{primaryColor};color:#fff;font-weight:600">Login</span>
          </div>
          <!-- Body -->
          <div style="padding:14px 12px;font-family:{fontFamily||'system-ui'},sans-serif">
            <!-- Title -->
            <div style="font-size:.65rem;text-transform:uppercase;letter-spacing:2px;color:{primaryColor};margin-bottom:6px;font-weight:600">Live Music · Warehouse District</div>
            <p style="font-size:.95rem;font-weight:800;color:#fff;margin:0 0 4px">{name || 'Venue Name'}</p>
            <p style="font-size:.7rem;color:#888;margin:0 0 12px;line-height:1.4">
              {venueDesc ? venueDesc.substring(0,80)+'...' : 'A converted 1920s cathedral. 800 capacity.'}
            </p>

            <!-- Event card -->
            <div style="background:{cardBg};border:1px solid {cardBorder};border-radius:8px;padding:10px;margin-bottom:8px">
              <div style="font-size:.65rem;text-transform:uppercase;letter-spacing:1px;color:{primaryColor};margin-bottom:3px;font-weight:600">Sat, Aug 15 · 8:00 PM</div>
              <p style="font-size:.8rem;font-weight:700;color:#fff;margin:0 0 6px">VOID DRIFTER</p>
              <div style="display:flex;align-items:center;justify-content:space-between;padding:8px;background:{bgColor};border-radius:6px;margin-bottom:4px">
                <div><p style="font-size:.7rem;color:#fff;margin:0 0 1px">GA</p><p style="font-size:.8rem;font-weight:700;color:{accentColor};margin:0">$25</p></div>
                <span style="font-size:.63rem;padding:4px 8px;background:{accentColor};color:#111;border-radius:4px;font-weight:700">Add</span>
              </div>
              <div style="display:flex;align-items:center;justify-content:space-between;padding:8px;background:{bgColor};border-radius:6px">
                <div><p style="font-size:.7rem;color:#fff;margin:0 0 1px">VIP</p><p style="font-size:.8rem;font-weight:700;color:{accentColor};margin:0">$75</p></div>
                <span style="font-size:.63rem;padding:4px 8px;background:{accentColor};color:#111;border-radius:4px;font-weight:700">Add</span>
              </div>
            </div>

            <!-- Total card -->
            <div style="background:{cardBg};border:1px solid {cardBorder};border-radius:8px;padding:10px;display:flex;justify-content:space-between;align-items:center">
              <span style="font-size:.75rem;font-weight:700;color:#fff">Total</span>
              <span style="font-size:.85rem;font-weight:800;color:{accentColor}">$100</span>
            </div>
          </div>
        </div>
        <p style="color:#555;font-size:.7rem;text-align:center;margin:0">Live preview of your public page</p>
      </div>
    </div>

  {:else if activeTab === 'custom-spa'}
    <!-- ===== CUSTOM SPA TAB ===== -->
    <div style="display:flex;flex-direction:column;gap:24px">
      <div style={cardStyle}>
        <div style="display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:12px">
          <div>
            <h3 style="font-size:1rem;font-weight:700;color:#fff;margin:0 0 4px">Custom SPA</h3>
            <p style="color:#888;font-size:.85rem;margin:0">
              {#if spaStatus?.has_custom_spa && spaStatus?.index_exists}
                <span style="color:#4ade80;font-weight:600">● Active</span>
                {#if spaStatus.spa_updated_at}
                  — updated {new Date(spaStatus.spa_updated_at).toLocaleDateString()}
                {/if}
              {:else}
                <span style="color:#888">○ No custom SPA uploaded</span>
              {/if}
            </p>
          </div>
          <div style="display:flex;gap:8px">
            {#if spaStatus?.has_custom_spa}
              <a href="http://localhost:3000/" target="_blank" rel="noopener"
                style="padding:8px 16px;background:#1a1a2e;border:1px solid #7c5ce7;color:#7c5ce7;text-decoration:none;border-radius:6px;font-size:.85rem;font-weight:600"
              >Preview →</a>
              <button onclick={handleSpaDelete}
                style="padding:8px 16px;background:transparent;border:1px solid #e74c3c;color:#e74c3c;border-radius:6px;cursor:pointer;font-size:.85rem;font-weight:600"
              >Remove</button>
            {/if}
          </div>
        </div>
      </div>

      <div style={cardStyle}>
        <h3 style="font-size:1rem;font-weight:700;color:#fff;margin:0 0 16px">Upload</h3>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:12px">
          <!-- Folder upload -->
          <div>
            <p style="color:#aaa;font-size:.8rem;margin:0 0 8px;font-weight:600">From folder (recommended)</p>
            <div
              style="border:2px dashed #2a2a4a;border-radius:12px;padding:24px;text-align:center;cursor:pointer;transition:border-color .2s"
              ondragover={(e) => { e.preventDefault(); }}
              onclick={() => (document.querySelector('.spa-folder-input') as HTMLInputElement)?.click()}
            >
              <input type="file" webkitdirectory multiple onchange={handleSpaFolderUpload} class="spa-folder-input" style="display:none" />
              <p style="color:#aaa;margin:0;font-size:.9rem"><span style="color:#7c5ce7;font-weight:600">Select folder</span></p>
              <p style="color:#555;margin:4px 0 0;font-size:.8rem">Pick your build/ directory</p>
            </div>
          </div>
          <!-- Tar.gz upload -->
          <div>
            <p style="color:#aaa;font-size:.8rem;margin:0 0 8px;font-weight:600">From .tar.gz archive</p>
            <div
              style="border:2px dashed #2a2a4a;border-radius:12px;padding:24px;text-align:center;cursor:pointer;transition:border-color .2s"
              ondragover={(e) => { e.preventDefault(); }}
              onclick={() => (document.querySelector('.spa-upload-input') as HTMLInputElement)?.click()}
            >
              <input type="file" accept=".tar.gz,application/gzip" onchange={handleSpaUpload} class="spa-upload-input" style="display:none" />
              <p style="color:#aaa;margin:0;font-size:.9rem"><span style="color:#7c5ce7;font-weight:600">Select .tar.gz</span></p>
              <p style="color:#555;margin:4px 0 0;font-size:.8rem">tar czf spa.tar.gz build/</p>
            </div>
          </div>
        </div>
        {#if spaUploadMsg}
          <p style="color:{spaUploadMsg.startsWith('Uploaded')||spaUploadMsg==='Removed.'?'#4ade80':'#e74c3c'};font-size:.9rem;margin:12px 0 0;font-weight:600">{spaUploadMsg}</p>
        {/if}
      </div>

      <div style={cardStyle}>
        <h3 style="font-size:1rem;font-weight:700;color:#fff;margin:0 0 12px">How it works</h3>
        <div style="color:#888;font-size:.85rem;line-height:1.7">
          <p style="margin:0 0 8px">1. Build your site with <code style="background:#0d0d1a;padding:2px 6px;border-radius:4px;color:#aaa">SvelteKit + adapter-static</code></p>
          <p style="margin:0 0 8px">2. Run <code style="background:#0d0d1a;padding:2px 6px;border-radius:4px;color:#aaa">npm run build</code> — produces a <code style="background:#0d0d1a;padding:2px 6px;border-radius:4px;color:#aaa">build/</code> directory</p>
          <p style="margin:0 0 8px">3. Tar it: <code style="background:#0d0d1a;padding:2px 6px;border-radius:4px;color:#aaa">tar czf spa.tar.gz build/</code></p>
          <p style="margin:0">4. Upload here. Your site replaces the public page at <code style="background:#0d0d1a;padding:2px 6px;border-radius:4px;color:#aaa">/spa/</code></p>
        </div>
        <p style="color:#555;font-size:.8rem;margin:12px 0 0">
          Your site receives <code style="background:#0d0d1a;padding:2px 6px;border-radius:4px;color:#777">window.RHYPH</code> with the organizer slug, name, and theme. Use the same <code style="background:#0d0d1a;padding:2px 6px;border-radius:4px;color:#777">/api/v1/</code> endpoints.
        </p>
      </div>
    </div>

  {:else}
    <!-- ===== DOMAIN TAB ===== -->
    <div style={cardStyle + 'text-align:center;padding:64px 24px'}>
      <div style="font-size:2.5rem;margin-bottom:16px">🌐</div>
      <h3 style="font-size:1.2rem;font-weight:700;color:#fff;margin:0 0 8px">Custom Domain</h3>
      <p style="color:#888;font-size:.95rem;margin:0 auto;max-width:450px;line-height:1.5">
        Point your own domain at this instance for full white-label ticketing. Coming in the next release.
      </p>
    </div>
  {/if}
</div>
