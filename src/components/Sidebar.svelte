<script lang="ts">
  import { t, type Language } from "../lib/i18n";

  export let language: Language = "en";
  export let mode: "centerline" | "outline" = "centerline";
  export let corners: "sharp" | "smooth" = "sharp";
  export let threshold: number = 210;
  export let autocrop: boolean = true;
  export let addFrame: boolean = false;
  export let isProcessing: boolean = false;

  export let onConvert: () => void;
  export let onParamChange: () => void;

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
  .checkbox-hint {
    font-size: 0.7rem;
    color: var(--text-muted);
    margin-left: 24px;
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
