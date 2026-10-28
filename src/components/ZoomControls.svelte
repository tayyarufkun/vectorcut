<script lang="ts">
  import { t, type Language } from "../lib/i18n";

  export let language: Language = "en";
  export let zoomBadgeText: string = "100%";
  export let onZoomIn: () => void;
  export let onZoomOut: () => void;
  export let onFit: () => void;
  export let onReset: () => void;
</script>

<div class="zoom-tools">
  <button class="zoom-btn" on:click={onZoomOut} title={t(language, "zoomOut")}>−</button>
  <span class="zoom-badge">{zoomBadgeText}</span>
  <button class="zoom-btn" on:click={onZoomIn} title={t(language, "zoomIn")}>+</button>
  <button class="zoom-btn btn-text" on:click={onFit} title={t(language, "zoomFit")}>⊡ {t(language, "zoomFit")}</button>
  <button class="zoom-btn btn-text" on:click={onReset} title={t(language, "zoomReset")}>1:1</button>
</div>

<style>
  .zoom-tools {
    display: inline-flex;
    align-items: center;
    gap: 3px;
    background: #e2e8f0;
    padding: 2px 4px;
    border-radius: 6px;
    user-select: none;
  }
  .zoom-btn {
    background: #ffffff;
    border: 1px solid #cbd5e1;
    border-radius: 4px;
    color: #1e293b;
    font-size: 0.75rem;
    font-weight: 700;
    padding: 2px 8px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.1s, border-color 0.1s;
    height: 22px;
    font-family: inherit;
  }
  .zoom-btn:hover {
    background: #f8fafc;
    border-color: #94a3b8;
  }
  .btn-text {
    font-size: 0.7rem;
    padding: 2px 6px;
  }
  .zoom-badge {
    font-size: 0.72rem;
    font-weight: 700;
    color: #475569;
    min-width: 44px;
    text-align: center;
    font-family: inherit;
  }
</style>
