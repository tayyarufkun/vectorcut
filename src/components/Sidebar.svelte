<script lang="ts">
  import { t, type Language } from "../lib/i18n";

  export let language: Language = "en";
  export let mode: "centerline" | "outline" = "centerline";
  export let corners: "sharp" | "smooth" = "sharp";
  export let threshold: number = 210;
  export let autocrop: boolean = true;
  export let addFrame: boolean = false;
  export let useAi: boolean = false;
  export let aiInstalled: boolean = false;
  export let aiDownloading: boolean = false;
  export let aiProgress: number = 0;
  export let isProcessing: boolean = false;

  export let onConvert: () => void;
  export let onParamChange: () => void;
  export let onDownloadAi: () => void;

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

  function toggleUseAi(val: boolean) {
    useAi = val;
    onParamChange();
  }
</script>

<div class="sidebar-content">
  <!-- Vektör Parametreleri -->
  <div class="card">
    <div class="section-title">{t(language, "parametersTitle")}</div>

    <!-- Mod Seçimi -->
    <div class="control-label">{t(language, "modeLabel")}</div>
    <div class="pill-group">
      <button
        class="pill-btn"
        class:active={mode === "centerline"}
        on:click={() => setMode("centerline")}
      >
        <div class="pill-header">
          <svg viewBox="0 0 24 24" class="pill-icon">
            <path d="M3 12h18v2H3z"/>
          </svg>
          <b>{t(language, "modeCenterline")}</b>
        </div>
        <span class="pill-desc">{t(language, "modeCenterlineSub")}</span>
      </button>
      <button
        class="pill-btn"
        class:active={mode === "outline"}
        on:click={() => setMode("outline")}
      >
        <div class="pill-header">
          <svg viewBox="0 0 24 24" class="pill-icon">
            <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z"/>
          </svg>
          <b>{t(language, "modeOutline")}</b>
        </div>
        <span class="pill-desc">{t(language, "modeOutlineSub")}</span>
      </button>
    </div>

    <!-- Köşe Tipi -->
    <div class="control-label" style="margin-top: 14px;">{t(language, "cornerLabel")}</div>
    <div class="pill-group">
      <button
        class="pill-btn"
        class:active={corners === "sharp"}
        on:click={() => setCorners("sharp")}
      >
        <div class="pill-header">
          <svg viewBox="0 0 24 24" class="pill-icon">
            <path d="M4 4v16h16v-2H6V4H4z"/>
          </svg>
          <b>{t(language, "cornerSharp")}</b>
        </div>
        <span class="pill-desc">{t(language, "cornerSharpSub")}</span>
      </button>
      <button
        class="pill-btn"
        class:active={corners === "smooth"}
        on:click={() => setCorners("smooth")}
      >
        <div class="pill-header">
          <svg viewBox="0 0 24 24" class="pill-icon">
            <path d="M4 18c4-12 12-12 16 0" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"/>
          </svg>
          <b>{t(language, "cornerSmooth")}</b>
        </div>
        <span class="pill-desc">{t(language, "cornerSmoothSub")}</span>
      </button>
    </div>

    <!-- Eşik Değeri (Threshold) -->
    <div style="margin-top: 16px;">
      <div class="slider-row">
        <label for="threshold-slider" class="control-label">{t(language, "thresholdLabel")}</label>
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
        <svg viewBox="0 0 24 24" class="check-opt-icon">
          <path d="M17 15h2V7c0-1.1-.9-2-2-2H9v2h8v8zM7 17V1H5v4H1v2h4v10c0 1.1.9 2 2 2h10v4h2v-4h4v-2H7z"/>
        </svg>
        <span>{t(language, "autoCropLabel")}</span>
      </label>
      <div class="checkbox-hint">
        {t(language, "autoCropHint")}
      </div>

      <label class="checkbox-label" style="margin-top: 6px;">
        <input
          type="checkbox"
          checked={addFrame}
          on:change={(e) => toggleAddFrame(e.currentTarget.checked)}
        />
        <svg viewBox="0 0 24 24" class="check-opt-icon">
          <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z"/>
        </svg>
        <span>{t(language, "addFrameLabel")}</span>
      </label>
      <div class="checkbox-hint">
        {t(language, "addFrameHint")}
      </div>
    </div>

    <!-- AI LineArt (Çok Renkli / Fotoğraf) -->
    <div class="checkbox-group ai-group">
      <label class="checkbox-label">
        <input
          type="checkbox"
          checked={useAi}
          on:change={(e) => toggleUseAi(e.currentTarget.checked)}
        />
        <svg viewBox="0 0 24 24" class="check-opt-icon ai-sparkle">
          <path d="M19 9l1.25-2.75L23 5l-2.75-1.25L19 1l-1.25 2.75L15 5l2.75 1.25L19 9zm-7.5.5L9 4 6.5 9.5 1 12l5.5 2.5L9 20l2.5-5.5L17 12l-5.5-2.5zM19 15l-1.25 2.75L15 19l2.75 1.25L19 23l1.25-2.75L23 19l-2.75-1.25L19 15z"/>
        </svg>
        <span class="ai-label-title">{t(language, "aiToggleLabel")}</span>
      </label>
      <div class="checkbox-hint">
        {t(language, "aiToggleHint")}
      </div>

      {#if useAi && !aiInstalled}
        <div class="ai-download-box">
          {#if aiDownloading}
            <div class="ai-progress-container">
              <div class="ai-progress-bar" style="width: {Math.max(5, Math.round(aiProgress * 100))}%;"></div>
            </div>
            <span class="ai-download-txt">{t(language, "prefAiDownloading")} {Math.round(aiProgress * 100)}%</span>
          {:else}
            <div class="ai-not-installed-msg">
              <span class="warn-badge">!</span>
              <span>{t(language, "aiModelRequired")}</span>
            </div>
            <button class="btn-ai-download" on:click={onDownloadAi}>
              <svg viewBox="0 0 24 24" class="download-icon">
                <path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/>
              </svg>
              <span>{t(language, "aiBtnDownloadModel")}</span>
            </button>
          {/if}
        </div>
      {/if}
    </div>
  </div>

  <!-- Aksiyon Butonu -->
  <button class="btn btn-convert" on:click={onConvert} disabled={isProcessing}>
    {#if isProcessing}
      <span class="spinner"></span>
      <span>{t(language, "btnGenerating")}</span>
    {:else}
      <svg viewBox="0 0 24 24" class="btn-icon">
        <path d="M8 5v14l11-7z"/>
      </svg>
      <span>{t(language, "btnGenerate")}</span>
    {/if}
  </button>
</div>

<style>
  .sidebar-content {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  .section-title {
    font-size: 0.8rem;
    text-transform: uppercase;
    color: var(--text-muted);
    font-weight: 700;
    letter-spacing: 0.5px;
    margin-bottom: 12px;
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
    color: var(--text);
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
    font-family: inherit;
  }
  .pill-header {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .pill-icon {
    width: 15px;
    height: 15px;
    fill: currentColor;
  }
  .pill-btn.active {
    background: var(--accent);
    color: #042f2e;
    border-color: var(--accent);
    box-shadow: 0 0 12px rgba(56, 189, 248, 0.25);
  }
  .pill-desc {
    font-size: 0.68rem;
    opacity: 0.85;
    margin-top: 3px;
    font-weight: 400;
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
  .check-opt-icon {
    width: 14px;
    height: 14px;
    fill: var(--text-muted);
  }
  .ai-sparkle {
    fill: #38bdf8;
  }
  .ai-label-title {
    color: #38bdf8;
    font-weight: 700;
  }
  .checkbox-hint {
    font-size: 0.7rem;
    color: var(--text-muted);
    margin-left: 24px;
  }

  .ai-group {
    background: rgba(56, 189, 248, 0.04);
    border-radius: 8px;
    padding: 10px;
    border: 1px solid rgba(56, 189, 248, 0.2);
    margin-top: 14px;
  }
  .ai-download-box {
    margin-top: 8px;
    padding: 10px;
    background: #0f172a;
    border: 1px dashed rgba(56, 189, 248, 0.4);
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .ai-not-installed-msg {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.75rem;
    color: #f59e0b;
  }
  .warn-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #f59e0b;
    color: #000;
    font-weight: bold;
    font-size: 0.7rem;
  }
  .btn-ai-download {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    background: rgba(56, 189, 248, 0.15);
    border: 1px solid var(--accent);
    color: var(--accent);
    border-radius: 6px;
    padding: 6px 10px;
    font-size: 0.76rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }
  .btn-ai-download:hover {
    background: var(--accent);
    color: #042f2e;
  }
  .download-icon {
    width: 14px;
    height: 14px;
    fill: currentColor;
  }
  .ai-progress-container {
    width: 100%;
    height: 6px;
    background: #1e293b;
    border-radius: 3px;
    overflow: hidden;
  }
  .ai-progress-bar {
    height: 100%;
    background: var(--accent);
    transition: width 0.2s ease;
  }
  .ai-download-txt {
    font-size: 0.72rem;
    color: var(--accent);
    text-align: center;
    font-weight: 600;
  }

  .btn {
    border: none;
    border-radius: 8px;
    padding: 12px;
    font-weight: 600;
    font-size: 0.88rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    transition: all 0.2s;
    user-select: none;
    font-family: inherit;
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
  .btn-icon {
    width: 18px;
    height: 18px;
    fill: currentColor;
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
