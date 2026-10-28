<script lang="ts">
  import { type AppPreferences, defaultPreferences } from "../lib/preferences";
  import { t } from "../lib/i18n";

  export let show: boolean = false;
  export let preferences: AppPreferences;
  export let onSave: (newPrefs: AppPreferences) => void;
  export let onClose: () => void;

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
    <div class="modal-card">
      <div class="modal-header">
        <div class="modal-title">
          <svg viewBox="0 0 24 24" class="title-icon">
            <path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/>
          </svg>
          <span>{t(localPrefs.language, "prefTitle")}</span>
        </div>
        <button class="close-btn" on:click={onClose} title="Close">✕</button>
      </div>

      <div class="modal-body">
        <!-- Dil Seçimi -->
        <div class="pref-group">
          <label class="pref-label" for="lang-select">{t(localPrefs.language, "prefLanguage")}</label>
          <div class="pill-group">
            <button
              class="pill-btn"
              class:active={localPrefs.language === "en"}
              on:click={() => (localPrefs.language = "en")}
            >
              <b>English</b>
              <span class="pill-sub">Default</span>
            </button>
            <button
              class="pill-btn"
              class:active={localPrefs.language === "tr"}
              on:click={() => (localPrefs.language = "tr")}
            >
              <b>Türkçe</b>
              <span class="pill-sub">Yerel</span>
            </button>
          </div>
        </div>

        <!-- Dosya Eki Ayarları -->
        <div class="pref-group">
          <div class="group-title">{t(localPrefs.language, "prefFileNaming")}</div>

          <div class="input-row">
            <label class="pref-label" for="centerline-suffix">{t(localPrefs.language, "prefCenterlineSuffix")}</label>
            <input
              id="centerline-suffix"
              type="text"
              class="text-input"
              bind:value={localPrefs.centerlineSuffix}
              placeholder="_centerline"
            />
            <span class="hint">{t(localPrefs.language, "prefCenterlineSuffixHint")}</span>
          </div>

          <div class="input-row" style="margin-top: 10px;">
            <label class="pref-label" for="outline-suffix">{t(localPrefs.language, "prefOutlineSuffix")}</label>
            <input
              id="outline-suffix"
              type="text"
              class="text-input"
              bind:value={localPrefs.outlineSuffix}
              placeholder="_outline"
            />
            <span class="hint">{t(localPrefs.language, "prefOutlineSuffixHint")}</span>
          </div>
        </div>

        <!-- Varsayılan Köşe -->
        <div class="pref-group">
          <div class="group-title">{t(localPrefs.language, "prefDefaults")}</div>
          <div class="pref-label">{t(localPrefs.language, "prefDefaultCorner")}</div>
          <div class="pill-group">
            <button
              class="pill-btn"
              class:active={localPrefs.defaultCorners === "sharp"}
              on:click={() => (localPrefs.defaultCorners = "sharp")}
            >
              <div class="pill-icon-text">
                <svg viewBox="0 0 24 24" class="option-icon">
                  <path d="M4 4v16h16v-2H6V4H4z"/>
                </svg>
                <span>{t(localPrefs.language, "cornerSharp")}</span>
              </div>
            </button>
            <button
              class="pill-btn"
              class:active={localPrefs.defaultCorners === "smooth"}
              on:click={() => (localPrefs.defaultCorners = "smooth")}
            >
              <div class="pill-icon-text">
                <svg viewBox="0 0 24 24" class="option-icon">
                  <path d="M4 18c4-12 12-12 16 0" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"/>
                </svg>
                <span>{t(localPrefs.language, "cornerSmooth")}</span>
              </div>
            </button>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn btn-secondary" on:click={handleReset}>
          {t(localPrefs.language, "prefReset")}
        </button>
        <button class="btn btn-primary" on:click={handleSave}>
          {t(localPrefs.language, "prefSave")}
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
    width: 480px;
    max-width: 90vw;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    font-family: inherit;
  }
  .modal-header {
    background: #0f172a;
    padding: 14px 20px;
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
  .title-icon {
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
    padding: 4px 8px;
    border-radius: 4px;
    transition: all 0.15s;
    font-family: inherit;
  }
  .close-btn:hover {
    color: var(--text);
    background: rgba(255, 255, 255, 0.1);
  }
  .modal-body {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 18px;
    max-height: 70vh;
    overflow-y: auto;
  }
  .pref-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .group-title {
    font-size: 0.8rem;
    text-transform: uppercase;
    color: var(--text-muted);
    font-weight: 700;
    letter-spacing: 0.5px;
    margin-bottom: 4px;
  }
  .pref-label {
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--text);
  }
  .input-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .text-input {
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 6px;
    padding: 8px 12px;
    color: var(--text);
    font-size: 0.85rem;
    outline: none;
    transition: border-color 0.15s;
    font-family: inherit;
  }
  .text-input:focus {
    border-color: var(--accent);
  }
  .hint {
    font-size: 0.72rem;
    color: var(--text-muted);
  }
  .pill-group {
    display: flex;
    gap: 8px;
  }
  .pill-btn {
    flex: 1;
    padding: 8px 12px;
    font-size: 0.85rem;
    border: 1px solid #334155;
    border-radius: 6px;
    background: #0f172a;
    color: var(--text);
    cursor: pointer;
    text-align: center;
    transition: all 0.15s;
    display: flex;
    flex-direction: column;
    align-items: center;
    font-family: inherit;
  }
  .pill-icon-text {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .option-icon {
    width: 14px;
    height: 14px;
    fill: currentColor;
  }
  .pill-btn.active {
    background: var(--accent);
    color: #042f2e;
    border-color: var(--accent);
    font-weight: 600;
  }
  .pill-sub {
    font-size: 0.7rem;
    opacity: 0.8;
  }
  .modal-footer {
    background: #0f172a;
    padding: 12px 20px;
    border-top: 1px solid #334155;
    display: flex;
    justify-content: flex-end;
    gap: 10px;
  }
  .btn {
    border: none;
    border-radius: 6px;
    padding: 8px 16px;
    font-size: 0.82rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s;
    font-family: inherit;
  }
  .btn-primary {
    background: var(--accent);
    color: #042f2e;
  }
  .btn-primary:hover {
    background: var(--accent-hover);
  }
  .btn-secondary {
    background: #334155;
    color: var(--text);
  }
  .btn-secondary:hover {
    background: #475569;
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
