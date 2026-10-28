<script lang="ts">
  import { t, type Language } from "../lib/i18n";

  export let language: Language = "en";
  export let viewMode: "split" | "overlay" = "split";
  export let hasSvg: boolean = false;
  export let toastMessage: string = "";
  export let showToast: boolean = false;

  export let onViewModeChange: (mode: "split" | "overlay") => void;
  export let onDownload: () => void;
  export let onOpenPreferences: () => void;
</script>

<header class="app-toolbar">
  <div class="brand">
    <svg viewBox="0 0 24 24" class="brand-icon">
      <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
    </svg>
    <div class="brand-text">
      <span class="title">{t(language, "appTitle")}</span>
      <span class="tag">{t(language, "appSubtitle")}</span>
    </div>
  </div>

  <div class="actions">
    {#if showToast}
      <div class="toast">
        <span class="toast-dot"></span>
        <span>{toastMessage}</span>
      </div>
    {/if}

    <div class="mode-toggles">
      <button
        class="toggle-btn"
        class:active={viewMode === "split"}
        on:click={() => onViewModeChange("split")}
      >
        {t(language, "viewSplit")}
      </button>
      <button
        class="toggle-btn"
        class:active={viewMode === "overlay"}
        on:click={() => onViewModeChange("overlay")}
      >
        {t(language, "viewOverlay")}
      </button>
    </div>

    <button
      class="btn btn-download"
      disabled={!hasSvg}
      on:click={onDownload}
      title="Save SVG file"
    >
      <svg viewBox="0 0 24 24" class="btn-icon">
        <path d="M19 12v7H5v-7H3v7c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-7h-2zm-6 .67l2.59-2.58L17 11.5l-5 5-5-5 1.41-1.41L11 12.67V3h2v9.67z"/>
      </svg>
      <span>{t(language, "exportSvg")}</span>
    </button>

    <button
      class="btn btn-icon-only"
      on:click={onOpenPreferences}
      title={t(language, "preferences")}
    >
      <svg viewBox="0 0 24 24" class="gear-icon">
        <path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/>
      </svg>
    </button>
  </div>
</header>

<style>
  .app-toolbar {
    background: var(--panel);
    border-bottom: 1px solid var(--panel-border);
    padding: 10px 20px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    user-select: none;
    z-index: 10;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .brand-icon {
    width: 26px;
    height: 26px;
    fill: var(--accent);
    filter: drop-shadow(0 0 8px rgba(56, 189, 248, 0.4));
  }
  .brand-text {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .title {
    font-weight: 700;
    font-size: 1.1rem;
    color: var(--text);
  }
  .tag {
    font-size: 0.72rem;
    background: rgba(56, 189, 248, 0.12);
    color: #7dd3fc;
    border: 1px solid rgba(56, 189, 248, 0.25);
    padding: 2px 8px;
    border-radius: 12px;
    font-weight: 600;
  }
  .actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .toast {
    display: flex;
    align-items: center;
    gap: 6px;
    background: rgba(16, 185, 129, 0.15);
    color: var(--success);
    border: 1px solid rgba(16, 185, 129, 0.3);
    padding: 5px 12px;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 600;
    animation: fadeIn 0.2s ease-out;
  }
  .toast-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--success);
    box-shadow: 0 0 6px var(--success);
  }
  .mode-toggles {
    display: flex;
    background: #0f172a;
    border: 1px solid var(--panel-border);
    border-radius: 6px;
    padding: 2px;
  }
  .toggle-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 0.78rem;
    padding: 5px 10px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s;
  }
  .toggle-btn:hover {
    color: var(--text);
  }
  .toggle-btn.active {
    background: #1e293b;
    color: var(--accent);
    font-weight: 700;
  }
  .btn {
    border: none;
    border-radius: 6px;
    padding: 7px 14px;
    font-size: 0.82rem;
    font-weight: 700;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 6px;
    transition: all 0.15s;
    user-select: none;
  }
  .btn-download {
    background: var(--accent);
    color: #042f2e;
    box-shadow: 0 2px 8px rgba(56, 189, 248, 0.2);
  }
  .btn-download:hover:not(:disabled) {
    background: var(--accent-hover);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(56, 189, 248, 0.35);
  }
  .btn-download:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background: #334155;
    color: #94a3b8;
    box-shadow: none;
  }
  .btn-icon-only {
    background: #1e293b;
    color: var(--text-muted);
    border: 1px solid var(--panel-border);
    padding: 7px 9px;
  }
  .btn-icon-only:hover {
    color: var(--text);
    border-color: var(--accent);
    background: #273549;
  }
  .btn-icon {
    width: 16px;
    height: 16px;
    fill: currentColor;
  }
  .gear-icon {
    width: 17px;
    height: 17px;
    fill: currentColor;
  }
  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-3px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
