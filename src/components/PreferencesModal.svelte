<script lang="ts">
  import { type AppPreferences, defaultPreferences } from "../lib/preferences";
  import { t } from "../lib/i18n";

  export let show: boolean = false;
  export let preferences: AppPreferences;
  export let aiInstalled: boolean = false;
  export let aiSizeMb: number = 0;
  export let aiPath: string = "";
  export let aiDownloading: boolean = false;
  export let aiProgress: number = 0;
  export let onSave: (newPrefs: AppPreferences) => void;
  export let onClose: () => void;
  export let onDownloadAi: () => void;
  export let onDeleteAi: () => void;

  type Tab = "general" | "export" | "ai" | "about";
  let activeTab: Tab = "general";

  let localPrefs: AppPreferences = { ...preferences };

  $: if (show) {
    localPrefs = { ...preferences };
  }

  function handleSave() {
    onSave(localPrefs);
    onClose();
  }

  function handleReset() {
    localPrefs = { ...defaultPreferences };
  }

  function handleBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }
</script>

{#if show}
  <div
    class="modal-backdrop"
    on:click={handleBackdrop}
    on:keydown={(e) => e.key === 'Escape' && onClose()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="settings-window">
      <!-- Sol Kenar Çubuğu (Navigation Sidebar) -->
      <aside class="settings-sidebar">
        <div class="sidebar-header">
          <div class="window-traffic-placeholder"></div>
          <div class="app-brand-row">
            <svg viewBox="0 0 24 24" class="brand-bolt">
              <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
            </svg>
            <span class="brand-title">{t(localPrefs.language, "prefTitle")}</span>
          </div>
        </div>

        <nav class="nav-menu">
          <!-- Genel -->
          <button
            class="nav-item"
            class:active={activeTab === "general"}
            on:click={() => (activeTab = "general")}
          >
            <svg viewBox="0 0 24 24" class="nav-icon">
              <path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/>
            </svg>
            <span>{t(localPrefs.language, "tabGeneral")}</span>
          </button>

          <!-- Dışa Aktarma -->
          <button
            class="nav-item"
            class:active={activeTab === "export"}
            on:click={() => (activeTab = "export")}
          >
            <svg viewBox="0 0 24 24" class="nav-icon">
              <path d="M19 12v7H5v-7H3v7c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-7h-2zm-6 .67l2.59-2.58L17 11.5l-5 5-5-5 1.41-1.41L11 12.67V3h2v9.67z"/>
            </svg>
            <span>{t(localPrefs.language, "tabExport")}</span>
          </button>

          <!-- AI Eklentisi -->
          <button
            class="nav-item"
            class:active={activeTab === "ai"}
            on:click={() => (activeTab = "ai")}
          >
            <svg viewBox="0 0 24 24" class="nav-icon ai-glow-icon">
              <path d="M19 9l1.25-2.75L23 5l-2.75-1.25L19 1l-1.25 2.75L15 5l2.75 1.25L19 9zm-7.5.5L9 4 6.5 9.5 1 12l5.5 2.5L9 20l2.5-5.5L17 12l-5.5-2.5zM19 15l-1.25 2.75L15 19l2.75 1.25L19 23l1.25-2.75L23 19l-2.75-1.25L19 15z"/>
            </svg>
            <span>{t(localPrefs.language, "tabAi")}</span>
            {#if aiInstalled}
              <span class="active-badge-dot"></span>
            {/if}
          </button>

          <!-- Hakkında -->
          <button
            class="nav-item"
            class:active={activeTab === "about"}
            on:click={() => (activeTab = "about")}
          >
            <svg viewBox="0 0 24 24" class="nav-icon">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/>
            </svg>
            <span>{t(localPrefs.language, "tabAbout")}</span>
          </button>
        </nav>

        <div class="sidebar-footer">
          <button class="btn-reset-ghost" on:click={handleReset}>
            {t(localPrefs.language, "prefReset")}
          </button>
        </div>
      </aside>

      <!-- Sağ Detay İçerik Alanı (Content View) -->
      <main class="settings-content">
        <header class="content-header">
          <h2 class="tab-heading">
            {#if activeTab === "general"}{t(localPrefs.language, "tabGeneral")}
            {:else if activeTab === "export"}{t(localPrefs.language, "tabExport")}
            {:else if activeTab === "ai"}{t(localPrefs.language, "tabAi")}
            {:else if activeTab === "about"}{t(localPrefs.language, "tabAbout")}
            {/if}
          </h2>
          <button class="btn-close-circle" on:click={onClose} title="Close (ESC)">✕</button>
        </header>

        <div class="content-body">
          <!-- 1. GENEL AYARLAR -->
          {#if activeTab === "general"}
            <div class="group-card">
              <!-- Dil Seçimi -->
              <div class="setting-row">
                <div class="setting-meta">
                  <span class="setting-title">{t(localPrefs.language, "prefLanguage")}</span>
                  <span class="setting-desc">Arayüz ve menü dilini anında değiştirir</span>
                </div>
                <div class="segmented-control">
                  <button
                    class="segment-btn"
                    class:active={localPrefs.language === "en"}
                    on:click={() => (localPrefs.language = "en")}
                  >
                    English
                  </button>
                  <button
                    class="segment-btn"
                    class:active={localPrefs.language === "tr"}
                    on:click={() => (localPrefs.language = "tr")}
                  >
                    Türkçe
                  </button>
                </div>
              </div>

              <div class="row-divider"></div>

              <!-- Varsayılan Köşe Stili -->
              <div class="setting-row">
                <div class="setting-meta">
                  <span class="setting-title">{t(localPrefs.language, "prefDefaultCorner")}</span>
                  <span class="setting-desc">Yeni açılan çizimlerde uygulanacak geometri</span>
                </div>
                <div class="segmented-control">
                  <button
                    class="segment-btn"
                    class:active={localPrefs.defaultCorners === "sharp"}
                    on:click={() => (localPrefs.defaultCorners = "sharp")}
                  >
                    <svg viewBox="0 0 24 24" class="btn-sm-icon"><path d="M4 4v16h16v-2H6V4H4z"/></svg>
                    <span>{t(localPrefs.language, "cornerSharp")}</span>
                  </button>
                  <button
                    class="segment-btn"
                    class:active={localPrefs.defaultCorners === "smooth"}
                    on:click={() => (localPrefs.defaultCorners = "smooth")}
                  >
                    <svg viewBox="0 0 24 24" class="btn-sm-icon"><path d="M4 18c4-12 12-12 16 0" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"/></svg>
                    <span>{t(localPrefs.language, "cornerSmooth")}</span>
                  </button>
                </div>
              </div>
            </div>

          <!-- 2. DIŞA AKTARMA (EXPORT) -->
          {:else if activeTab === "export"}
            <div class="group-card">
              <div class="setting-row column-layout">
                <div class="setting-meta">
                  <span class="setting-title">{t(localPrefs.language, "prefCenterlineSuffix")}</span>
                  <span class="setting-desc">{t(localPrefs.language, "prefCenterlineSuffixHint")}</span>
                </div>
                <div class="input-with-preview">
                  <input
                    type="text"
                    class="modern-input"
                    bind:value={localPrefs.centerlineSuffix}
                    placeholder="_centerline"
                  />
                  <span class="preview-badge">drawing{localPrefs.centerlineSuffix || ""}.svg</span>
                </div>
              </div>

              <div class="row-divider"></div>

              <div class="setting-row column-layout">
                <div class="setting-meta">
                  <span class="setting-title">{t(localPrefs.language, "prefOutlineSuffix")}</span>
                  <span class="setting-desc">{t(localPrefs.language, "prefOutlineSuffixHint")}</span>
                </div>
                <div class="input-with-preview">
                  <input
                    type="text"
                    class="modern-input"
                    bind:value={localPrefs.outlineSuffix}
                    placeholder="_outline"
                  />
                  <span class="preview-badge">drawing{localPrefs.outlineSuffix || ""}.svg</span>
                </div>
              </div>
            </div>

          <!-- 3. AI EKLENTİSİ (AI EXTENSION) -->
          {:else if activeTab === "ai"}
            <div class="group-card">
              <div class="ai-hero-status">
                <div class="status-pulse-wrap">
                  <div class="pulse-dot" class:online={aiInstalled} class:downloading={aiDownloading}></div>
                </div>
                <div class="ai-hero-details">
                  <h4 class="ai-engine-title">
                    {#if aiInstalled}
                      {t(localPrefs.language, "prefAiStatusReady")}
                    {:else if aiDownloading}
                      {t(localPrefs.language, "prefAiDownloading")}
                    {:else}
                      {t(localPrefs.language, "prefAiStatusNotInstalled")}
                    {/if}
                  </h4>
                  <p class="ai-engine-sub">
                    {t(localPrefs.language, "prefAiDescription")}
                  </p>
                  {#if aiInstalled}
                    <div class="engine-meta-tags">
                      <span class="meta-tag" title={aiPath}>Disk: {aiSizeMb.toFixed(1)} MB</span>
                      <span class="meta-tag">Pure Rust (tract-onnx)</span>
                      <span class="meta-tag">CPU Accelerated</span>
                    </div>
                  {/if}
                </div>
              </div>

              {#if aiDownloading}
                <div class="ai-download-block">
                  <div class="download-progress-track">
                    <div class="download-progress-bar" style="width: {Math.max(5, Math.round(aiProgress * 100))}%;"></div>
                  </div>
                  <div class="download-info-row">
                    <span>Model weights downloading via CDN...</span>
                    <span class="pct-num">{Math.round(aiProgress * 100)}%</span>
                  </div>
                </div>
              {:else}
                <div class="ai-actions-row">
                  {#if aiInstalled}
                    <button class="btn-danger-glass" on:click={onDeleteAi}>
                      <svg viewBox="0 0 24 24" class="btn-sm-icon"><path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
                      <span>{t(localPrefs.language, "prefAiDeleteBtn")}</span>
                    </button>
                  {:else}
                    <button class="btn-accent-cta" on:click={onDownloadAi}>
                      <svg viewBox="0 0 24 24" class="btn-sm-icon"><path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/></svg>
                      <span>{t(localPrefs.language, "prefAiDownloadBtn")}</span>
                    </button>
                  {/if}
                </div>
              {/if}
            </div>

            <div class="feature-bullets-card">
              <div class="bullet-item">
                <span class="bullet-check">✓</span>
                <span><b>Zero Dependency:</b> Runs locally with pure Rust without Python, CUDA or C++ DLLs.</span>
              </div>
              <div class="bullet-item">
                <span class="bullet-check">✓</span>
                <span><b>Letterboxed Geometry:</b> Preserves original aspect ratios and keeps circles perfectly circular.</span>
              </div>
              <div class="bullet-item">
                <span class="bullet-check">✓</span>
                <span><b>Smart Fallback:</b> Automatically applies Sobel gradient detection if model is uninstalled.</span>
              </div>
            </div>

          <!-- 4. HAKKINDA (ABOUT) -->
          {:else if activeTab === "about"}
            <div class="about-card">
              <div class="about-logo-wrap">
                <svg viewBox="0 0 24 24" class="about-large-bolt">
                  <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
                </svg>
              </div>
              <h3 class="about-app-name">VectorCut Studio</h3>
              <span class="about-version-badge">v1.1.0 (Universal Release)</span>
              <p class="about-desc-text">
                {t(localPrefs.language, "prefAboutDesc")}
              </p>
              <div class="about-links-row">
                <a
                  href="https://github.com/tayyarufkun/vectorcut"
                  target="_blank"
                  rel="noreferrer"
                  class="about-link-pill"
                >
                  <svg viewBox="0 0 24 24" class="btn-sm-icon"><path d="M12 2A10 10 0 0 0 2 12c0 4.42 2.87 8.17 6.84 9.5.5.08.66-.23.66-.5v-1.69c-2.77.6-3.36-1.34-3.36-1.34-.46-1.16-1.11-1.47-1.11-1.47-.91-.62.07-.6.07-.6 1 .07 1.53 1.03 1.53 1.03.87 1.52 2.34 1.07 2.91.83.09-.65.35-1.09.63-1.34-2.22-.25-4.55-1.11-4.55-4.92 0-1.11.38-2 1.03-2.71-.1-.25-.45-1.29.1-2.64 0 0 .84-.27 2.75 1.02.79-.22 1.65-.33 2.5-.33.85 0 1.71.11 2.5.33 1.91-1.29 2.75-1.02 2.75-1.02.55 1.35.2 2.39.1 2.64.65.71 1.03 1.6 1.03 2.71 0 3.82-2.34 4.66-4.57 4.91.36.31.69.92.69 1.85V21c0 .27.16.59.67.5C19.14 20.16 22 16.42 22 12A10 10 0 0 0 12 2z"/></svg>
                  <span>{t(localPrefs.language, "prefAboutGithub")}</span>
                </a>
                <span class="license-tag">{t(localPrefs.language, "prefAboutLicense")}</span>
              </div>
            </div>
          {/if}
        </div>

        <!-- Alt Aksiyon Barı (Footer) -->
        <footer class="content-footer">
          <button class="btn-save-pill" on:click={handleSave}>
            {t(localPrefs.language, "prefSave")}
          </button>
        </footer>
      </main>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(4, 8, 16, 0.72);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
    animation: fadeIn 0.15s ease-out;
  }

  .settings-window {
    background: #0d131f;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 14px;
    width: 660px;
    height: 480px;
    max-width: 92vw;
    max-height: 85vh;
    display: flex;
    overflow: hidden;
    box-shadow: 0 30px 70px rgba(0, 0, 0, 0.7), 0 0 1px rgba(255, 255, 255, 0.2);
    font-family: inherit;
  }

  /* SOL KENAR ÇUBUĞU */
  .settings-sidebar {
    width: 195px;
    min-width: 195px;
    background: rgba(15, 23, 42, 0.85);
    border-right: 1px solid rgba(255, 255, 255, 0.07);
    display: flex;
    flex-direction: column;
    padding: 16px 12px;
  }

  .sidebar-header {
    margin-bottom: 20px;
    padding-left: 6px;
  }

  .app-brand-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .brand-bolt {
    width: 18px;
    height: 18px;
    fill: var(--accent);
  }

  .brand-title {
    font-size: 0.88rem;
    font-weight: 700;
    color: var(--text);
    letter-spacing: 0.3px;
  }

  .nav-menu {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    border-radius: 8px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    font-size: 0.84rem;
    font-weight: 500;
    cursor: pointer;
    text-align: left;
    transition: all 0.15s ease;
    user-select: none;
    position: relative;
    font-family: inherit;
  }

  .nav-item:hover {
    background: rgba(255, 255, 255, 0.05);
    color: var(--text);
  }

  .nav-item.active {
    background: rgba(56, 189, 248, 0.15);
    color: var(--accent);
    font-weight: 600;
  }

  .nav-icon {
    width: 16px;
    height: 16px;
    fill: currentColor;
    flex-shrink: 0;
  }

  .ai-glow-icon {
    fill: #38bdf8;
  }

  .active-badge-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #10b981;
    margin-left: auto;
    box-shadow: 0 0 6px #10b981;
  }

  .sidebar-footer {
    padding-top: 10px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
  }

  .btn-reset-ghost {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 0.74rem;
    cursor: pointer;
    padding: 6px 8px;
    border-radius: 6px;
    transition: all 0.15s;
    width: 100%;
    text-align: left;
    font-family: inherit;
  }

  .btn-reset-ghost:hover {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.08);
  }

  /* SAĞ İÇERİK ALANI */
  .settings-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: #0f172a;
    overflow: hidden;
  }

  .content-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  }

  .tab-heading {
    font-size: 1.05rem;
    font-weight: 700;
    color: var(--text);
  }

  .btn-close-circle {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    border: none;
    background: rgba(255, 255, 255, 0.06);
    color: var(--text-muted);
    font-size: 0.85rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-close-circle:hover {
    background: rgba(255, 255, 255, 0.15);
    color: var(--text);
  }

  .content-body {
    flex: 1;
    padding: 18px 20px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  /* GRUP KARTLARI */
  .group-card {
    background: #182234;
    border: 1px solid rgba(255, 255, 255, 0.07);
    border-radius: 10px;
    padding: 14px 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .setting-row.column-layout {
    flex-direction: column;
    align-items: flex-start;
  }

  .setting-meta {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .setting-title {
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text);
  }

  .setting-desc {
    font-size: 0.72rem;
    color: var(--text-muted);
    line-height: 1.35;
  }

  .row-divider {
    height: 1px;
    background: rgba(255, 255, 255, 0.05);
    margin: 2px 0;
  }

  /* SEGMENTED CONTROL (Apple style) */
  .segmented-control {
    display: flex;
    background: #0f172a;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    padding: 3px;
    gap: 3px;
  }

  .segment-btn {
    border: none;
    background: transparent;
    color: var(--text-muted);
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 0.78rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    display: flex;
    align-items: center;
    gap: 6px;
    font-family: inherit;
    user-select: none;
  }

  .segment-btn:hover {
    color: var(--text);
  }

  .segment-btn.active {
    background: var(--accent);
    color: #042f2e;
    font-weight: 700;
    box-shadow: 0 1px 6px rgba(0, 0, 0, 0.3);
  }

  .btn-sm-icon {
    width: 13px;
    height: 13px;
    fill: currentColor;
  }

  /* INPUT WITH PREVIEW */
  .input-with-preview {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    margin-top: 4px;
  }

  .modern-input {
    flex: 1;
    background: #0f172a;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    padding: 8px 12px;
    color: var(--text);
    font-size: 0.82rem;
    outline: none;
    transition: border-color 0.15s;
    font-family: inherit;
  }

  .modern-input:focus {
    border-color: var(--accent);
  }

  .preview-badge {
    background: rgba(56, 189, 248, 0.08);
    border: 1px solid rgba(56, 189, 248, 0.2);
    color: var(--accent);
    font-family: monospace;
    font-size: 0.74rem;
    padding: 6px 10px;
    border-radius: 6px;
    white-space: nowrap;
  }

  /* AI HERO KARTI */
  .ai-hero-status {
    display: flex;
    gap: 14px;
    align-items: flex-start;
  }

  .status-pulse-wrap {
    padding-top: 4px;
  }

  .pulse-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: #64748b;
  }

  .pulse-dot.online {
    background: #10b981;
    box-shadow: 0 0 10px #10b981;
  }

  .pulse-dot.downloading {
    background: var(--accent);
    box-shadow: 0 0 10px var(--accent);
    animation: blink 1s infinite alternate;
  }

  .ai-hero-details {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
  }

  .ai-engine-title {
    font-size: 0.9rem;
    font-weight: 700;
    color: var(--text);
  }

  .ai-engine-sub {
    font-size: 0.74rem;
    color: var(--text-muted);
    line-height: 1.4;
  }

  .engine-meta-tags {
    display: flex;
    gap: 6px;
    margin-top: 6px;
    flex-wrap: wrap;
  }

  .meta-tag {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: #cbd5e1;
    font-size: 0.68rem;
    padding: 2px 7px;
    border-radius: 4px;
  }

  .ai-download-block {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: 6px;
  }

  .download-progress-track {
    width: 100%;
    height: 6px;
    background: #0f172a;
    border-radius: 3px;
    overflow: hidden;
  }

  .download-progress-bar {
    height: 100%;
    background: var(--accent);
    transition: width 0.15s ease;
  }

  .download-info-row {
    display: flex;
    justify-content: space-between;
    font-size: 0.72rem;
    color: var(--accent);
    font-weight: 600;
  }

  .ai-actions-row {
    margin-top: 6px;
    display: flex;
    justify-content: flex-end;
  }

  .btn-accent-cta {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: var(--accent);
    color: #042f2e;
    border: none;
    border-radius: 6px;
    padding: 7px 14px;
    font-size: 0.8rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.15s;
    font-family: inherit;
  }

  .btn-accent-cta:hover {
    background: var(--accent-hover);
    transform: translateY(-1px);
  }

  .btn-danger-glass {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: rgba(239, 68, 68, 0.08);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #ef4444;
    border-radius: 6px;
    padding: 6px 12px;
    font-size: 0.78rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s;
    font-family: inherit;
  }

  .btn-danger-glass:hover {
    background: #ef4444;
    color: #fff;
  }

  .feature-bullets-card {
    background: rgba(15, 23, 42, 0.5);
    border: 1px dashed rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .bullet-item {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    font-size: 0.73rem;
    color: var(--text-muted);
    line-height: 1.35;
  }

  .bullet-check {
    color: #10b981;
    font-weight: 700;
  }

  /* HAKKINDA KARTI */
  .about-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 24px 16px;
    background: #182234;
    border: 1px solid rgba(255, 255, 255, 0.07);
    border-radius: 12px;
  }

  .about-logo-wrap {
    width: 52px;
    height: 52px;
    border-radius: 14px;
    background: rgba(56, 189, 248, 0.12);
    border: 1px solid rgba(56, 189, 248, 0.3);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 12px;
  }

  .about-large-bolt {
    width: 28px;
    height: 28px;
    fill: var(--accent);
  }

  .about-app-name {
    font-size: 1.15rem;
    font-weight: 800;
    color: var(--text);
  }

  .about-version-badge {
    display: inline-block;
    background: rgba(56, 189, 248, 0.15);
    color: var(--accent);
    font-size: 0.72rem;
    font-weight: 700;
    padding: 3px 8px;
    border-radius: 12px;
    margin-top: 4px;
    margin-bottom: 12px;
  }

  .about-desc-text {
    font-size: 0.78rem;
    color: var(--text-muted);
    max-width: 360px;
    line-height: 1.45;
    margin-bottom: 18px;
  }

  .about-links-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .about-link-pill {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: #0f172a;
    border: 1px solid rgba(255, 255, 255, 0.12);
    color: var(--text);
    text-decoration: none;
    font-size: 0.76rem;
    font-weight: 600;
    padding: 6px 12px;
    border-radius: 6px;
    transition: all 0.15s;
  }

  .about-link-pill:hover {
    border-color: var(--accent);
    color: var(--accent);
  }

  .license-tag {
    font-size: 0.72rem;
    color: var(--text-muted);
  }

  /* ALT AKSİYON BARI */
  .content-footer {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    padding: 12px 20px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
    background: rgba(15, 23, 42, 0.5);
  }

  .btn-save-pill {
    background: var(--accent);
    color: #042f2e;
    border: none;
    border-radius: 6px;
    padding: 7px 18px;
    font-size: 0.82rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.15s ease;
    font-family: inherit;
    box-shadow: 0 2px 8px rgba(56, 189, 248, 0.25);
  }

  .btn-save-pill:hover {
    background: var(--accent-hover);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(56, 189, 248, 0.35);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: scale(0.98);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  @keyframes blink {
    from {
      opacity: 0.4;
    }
    to {
      opacity: 1;
    }
  }
</style>
