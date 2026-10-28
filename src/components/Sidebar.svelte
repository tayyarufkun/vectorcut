<script lang="ts">
  import { onMount } from "svelte";
  import { listQuickFolders } from "../lib/api";

  export let mode: "centerline" | "outline" = "centerline";
  export let corners: "sharp" | "smooth" = "sharp";
  export let threshold: number = 210;
  export let autocrop: boolean = true;
  export let addFrame: boolean = false;
  export let isProcessing: boolean = false;

  export let onConvert: () => void;
  export let onParamChange: () => void;
  export let onFolderClick: (folder: string) => void;

  let quickFolders: string[] = ["Teleferik", "Vosvos", "Kauçuk Baskı"];

  onMount(async () => {
    try {
      const folders = await listQuickFolders();
      if (folders && folders.length > 0) {
        quickFolders = folders;
      }
    } catch {
      // Fallback
    }
  });

  function setMode(m: "centerline" | "outline") {
    mode = m;
    onParamChange();
  }

  function setCorners(c: "sharp" | "smooth") {
    corners = c;
    onParamChange();
  }

  function handleThreshold(val: number) {
    threshold = val;
    onParamChange();
  }

  function toggleAutoCrop(val: boolean) {
    autocrop = val;
    onParamChange();
  }

  function toggleAddFrame(val: boolean) {
    addFrame = val;
    onParamChange();
  }
</script>

<div class="sidebar-content">
  <!-- Hızlı Masaüstü Klasörleri -->
  {#if quickFolders.length > 0}
    <div class="sidebar-section">
      <div class="section-title">Hızlı Masaüstü Klasörleri</div>
      <div class="folder-list">
        {#each quickFolders as folder}
          <button class="folder-btn" on:click={() => onFolderClick(folder)}>
            📁 {folder}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Kesim Parametreleri -->
  <div class="card">
    <div class="section-title" style="margin-bottom: 12px;">Kesim Parametreleri</div>

    <!-- Kesim Modu -->
    <div class="control-label">Kesim Hattı Modu:</div>
    <div class="pill-group">
      <button
        class="pill-btn"
        class:active={mode === "centerline"}
        on:click={() => setMode("centerline")}
      >
        <b>Tek Hat</b>
        <span class="pill-desc">Centerline (Çizgiler)</span>
      </button>
      <button
        class="pill-btn"
        class:active={mode === "outline"}
        on:click={() => setMode("outline")}
      >
        <b>Dış Kontur</b>
        <span class="pill-desc">Outline (Silüetler)</span>
      </button>
    </div>

    <!-- Köşe Tipi -->
    <div class="control-label" style="margin-top: 14px;">Köşe Yapısı:</div>
    <div class="pill-group">
      <button
        class="pill-btn"
        class:active={corners === "sharp"}
        on:click={() => setCorners("sharp")}
      >
        <b>📐 Dik Köşeler</b>
        <span class="pill-desc">Kutu, Çerçeve, Pencere</span>
      </button>
      <button
        class="pill-btn"
        class:active={corners === "smooth"}
        on:click={() => setCorners("smooth")}
      >
        <b>🌀 Eğrisel / Yay</b>
        <span class="pill-desc">Organik Hatlar & Yaylar</span>
      </button>
    </div>

    <!-- Eşik Değeri -->
    <div style="margin-top: 16px;">
      <div class="slider-row">
        <label for="threshold-slider" class="control-label">Siyah/Beyaz Hassasiyeti:</label>
        <span class="threshold-value">{threshold}</span>
      </div>
      <input
        id="threshold-slider"
        type="range"
        min="50"
        max="254"
        value={threshold}
        on:input={(e) => handleThreshold(Number(e.currentTarget.value))}
      />
    </div>

    <!-- Çerçeve & Kırpma Seçenekleri -->
    <div class="checkbox-group">
      <label class="checkbox-label">
        <input
          type="checkbox"
          checked={autocrop}
          on:change={(e) => toggleAutoCrop(e.currentTarget.checked)}
        />
        <span>✂️ Boşlukları Kırp (Sıkı Çerçeve)</span>
      </label>
      <div class="checkbox-hint">
        Dış boşlukları temizler, SVG sınırını çizimin en dış noktalarına sıfırlar.
      </div>

      <label class="checkbox-label" style="margin-top: 6px;">
        <input
          type="checkbox"
          checked={addFrame}
          on:change={(e) => toggleAddFrame(e.currentTarget.checked)}
        />
        <span>🔲 Dış Kesim Çerçevesi Ekle</span>
      </label>
      <div class="checkbox-hint">
        Şeklin en dış noktalarını çevreleyen bir kesim dikdörtgeni ekler.
      </div>
    </div>
  </div>

  <!-- Aksiyon Butonu -->
  <button class="btn btn-convert" on:click={onConvert} disabled={isProcessing}>
    {#if isProcessing}
      <span class="spinner"></span>
      <span>Dönüştürülüyor...</span>
    {:else}
      <span>⚡️ Tek Hat SVG Oluştur</span>
    {/if}
  </button>
</div>

<style>
  .sidebar-content {
    display: flex;
    flex-direction: column;
    gap: 18px;
  }
  .sidebar-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .section-title {
    font-size: 0.82rem;
    text-transform: uppercase;
    color: var(--text-muted);
    font-weight: 700;
    letter-spacing: 0.5px;
  }
  .folder-list {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }
  .folder-btn {
    background: #0f172a;
    border: 1px solid var(--panel-border);
    color: var(--text);
    font-size: 0.8rem;
    padding: 6px 10px;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.15s;
    user-select: none;
  }
  .folder-btn:hover {
    border-color: var(--accent);
    color: var(--accent);
    background: rgba(56, 189, 248, 0.05);
  }
  .card {
    background: #182234;
    border: 1px solid var(--panel-border);
    border-radius: 10px;
    padding: 16px;
  }
  .control-label {
    font-size: 0.8rem;
    font-weight: 600;
    display: block;
    margin-bottom: 6px;
  }
  .pill-group {
    display: flex;
    gap: 8px;
  }
  .pill-btn {
    flex: 1;
    padding: 9px 8px;
    font-size: 0.85rem;
    border: 1px solid var(--panel-border);
    border-radius: 8px;
    background: #111827;
    color: var(--text);
    cursor: pointer;
    text-align: center;
    transition: all 0.15s;
    user-select: none;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .pill-btn.active {
    background: var(--accent);
    color: #042f2e;
    border-color: var(--accent);
    box-shadow: 0 0 12px rgba(56, 189, 248, 0.25);
  }
  .pill-desc {
    font-size: 0.7rem;
    opacity: 0.85;
    margin-top: 2px;
  }
  .slider-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 6px;
  }
  .threshold-value {
    font-size: 0.85rem;
    font-weight: 700;
    color: var(--accent);
  }
  input[type="range"] {
    width: 100%;
    accent-color: var(--accent);
    cursor: pointer;
  }
  .checkbox-group {
    margin-top: 14px;
    padding-top: 12px;
    border-top: 1px solid var(--panel-border);
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    user-select: none;
  }
  .checkbox-label input[type="checkbox"] {
    accent-color: var(--accent);
    width: 16px;
    height: 16px;
    cursor: pointer;
  }
  .checkbox-hint {
    font-size: 0.7rem;
    color: var(--text-muted);
    margin-left: 24px;
  }
  .btn {
    border: none;
    border-radius: 8px;
    padding: 13px;
    font-weight: 700;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    transition: all 0.2s;
    user-select: none;
  }
  .btn-convert {
    background: var(--accent);
    color: #042f2e;
    box-shadow: 0 4px 14px rgba(56, 189, 248, 0.3);
  }
  .btn-convert:hover:not(:disabled) {
    background: var(--accent-hover);
    transform: translateY(-1px);
    box-shadow: 0 6px 18px rgba(56, 189, 248, 0.4);
  }
  .btn-convert:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid #042f2e;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
