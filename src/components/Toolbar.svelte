<script lang="ts">
  export let viewMode: "split" | "overlay" = "split";
  export let hasSvg: boolean = false;
  export let toastMessage: string = "";
  export let showToast: boolean = false;

  export let onViewModeChange: (mode: "split" | "overlay") => void;
  export let onDownload: () => void;
</script>

<header class="app-toolbar">
  <div class="brand">
    <svg viewBox="0 0 24 24" class="brand-icon">
      <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
    </svg>
    <div class="brand-text">
      <span class="title">VectorCut Studio</span>
      <span class="tag">Snapmaker Luban Uyumlu</span>
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
        Yan Yana
      </button>
      <button
        class="toggle-btn"
        class:active={viewMode === "overlay"}
        on:click={() => onViewModeChange("overlay")}
      >
        Üst Üste Bindir (Lazer Kontrolü)
      </button>
    </div>

    <button
      class="btn btn-download"
      disabled={!hasSvg}
      on:click={onDownload}
      title="Yerel Farklı Kaydet penceresini açar"
    >
      <svg viewBox="0 0 24 24" class="dl-icon">
        <path d="M19 12v7H5v-7H3v7c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-7h-2zm-6 .67l2.59-2.58L17 11.5l-5 5-5-5 1.41-1.41L11 12.67V3h2v9.67z"/>
      </svg>
      <span>Luban SVG İndir</span>
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
    gap: 8px;
  }
  .title {
    font-weight: 700;
    font-size: 1.1rem;
    color: var(--text);
  }
  .tag {
    font-size: 0.72rem;
    background: rgba(3, 105, 161, 0.35);
    color: #7dd3fc;
    border: 1px solid rgba(56, 189, 248, 0.3);
    padding: 2px 8px;
    border-radius: 12px;
    font-weight: 600;
  }
  .actions {
    display: flex;
    align-items: center;
    gap: 12px;
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
  .dl-icon {
    width: 16px;
    height: 16px;
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
