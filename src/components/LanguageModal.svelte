<script lang="ts">
  import { t, type Language } from "../lib/i18n";

  export let show: boolean = false;
  export let currentLanguage: Language = "en";
  export let onSelectLanguage: (lang: Language) => void;
  export let onClose: () => void;

  function select(lang: Language) {
    onSelectLanguage(lang);
    onClose();
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
    <div class="modal-card">
      <div class="modal-header">
        <div class="modal-title">
          <svg viewBox="0 0 24 24" class="globe-icon">
            <path d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zm6.93 6h-2.95a15.65 15.65 0 0 0-1.38-3.56A8.03 8.03 0 0 1 18.92 8zM12 4.04c.83 1.2 1.48 2.53 1.91 3.96h-3.82c.43-1.43 1.08-2.76 1.91-3.96zM4.26 14C4.1 13.36 4 12.69 4 12s.1-1.36.26-2h3.38c-.08.66-.14 1.32-.14 2 0 .68.06 1.34.14 2H4.26zm.82 2h2.95c.32 1.25.78 2.45 1.38 3.56A7.987 7.987 0 0 1 5.08 16zm2.95-8H5.08a7.987 7.987 0 0 1 3.95-3.56A15.65 15.65 0 0 0 8.03 8zm4.97 11.96c-.83-1.2-1.48-2.53-1.91-3.96h3.82c-.43 1.43-1.08 2.76-1.91 3.96zM14.34 14H9.66c-.09-.66-.16-1.32-.16-2 0-.68.07-1.35.16-2h4.68c.09.65.16 1.32.16 2 0 .68-.07 1.34-.16 2zm.79 5.56c.6-1.11 1.06-2.31 1.38-3.56h2.95a8.03 8.03 0 0 1-4.33 3.56zM16.36 14c.08-.66.14-1.32.14-2 0-.68-.06-1.34-.14-2h3.38c.16.64.26 1.31.26 2s-.1 1.36-.26 2h-3.38z"/>
          </svg>
          <span>{t(currentLanguage, "languageModalTitle")}</span>
        </div>
        <button class="close-btn" on:click={onClose} title="Close">✕</button>
      </div>

      <div class="lang-list">
        <button
          class="lang-item"
          class:selected={currentLanguage === "en"}
          on:click={() => select("en")}
        >
          <div class="lang-text">
            <span class="lang-name">English</span>
            <span class="lang-region">Default</span>
          </div>
          {#if currentLanguage === "en"}
            <svg viewBox="0 0 24 24" class="check-icon">
              <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
            </svg>
          {/if}
        </button>

        <button
          class="lang-item"
          class:selected={currentLanguage === "tr"}
          on:click={() => select("tr")}
        >
          <div class="lang-text">
            <span class="lang-name">Türkçe</span>
            <span class="lang-region">Yerel</span>
          </div>
          {#if currentLanguage === "tr"}
            <svg viewBox="0 0 24 24" class="check-icon">
              <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
            </svg>
          {/if}
        </button>
      </div>
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
    background: rgba(0, 0, 0, 0.65);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.15s ease-out;
  }
  .modal-card {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 12px;
    width: 380px;
    max-width: 90vw;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .modal-header {
    background: #0f172a;
    padding: 14px 18px;
    border-bottom: 1px solid #334155;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .modal-title {
    display: flex;
    align-items: center;
    gap: 10px;
    font-weight: 700;
    font-size: 0.95rem;
    color: var(--text);
  }
  .globe-icon {
    width: 20px;
    height: 20px;
    fill: var(--accent);
  }
  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 1.1rem;
    cursor: pointer;
    padding: 2px 6px;
    border-radius: 4px;
    transition: all 0.15s;
  }
  .close-btn:hover {
    color: var(--text);
    background: rgba(255, 255, 255, 0.1);
  }
  .lang-list {
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .lang-item {
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 8px;
    padding: 12px 16px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    transition: all 0.15s;
    text-align: left;
    color: var(--text);
  }
  .lang-item:hover {
    border-color: var(--accent);
    background: #172554;
  }
  .lang-item.selected {
    border-color: var(--accent);
    background: rgba(56, 189, 248, 0.12);
  }
  .lang-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .lang-name {
    font-weight: 600;
    font-size: 0.92rem;
  }
  .lang-region {
    font-size: 0.72rem;
    color: var(--text-muted);
  }
  .check-icon {
    width: 20px;
    height: 20px;
    fill: var(--accent);
  }
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
</style>
