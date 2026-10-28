<script lang="ts">
  import type { PanZoomViewer } from "../lib/panzoom";
  import { t, type Language } from "../lib/i18n";

  export let language: Language = "en";
  export let viewer: PanZoomViewer | null = null;
  export let zoomBadgeText: string = "100%";

  function zoomOut() {
    viewer?.zoomStep(-1);
  }

  function zoomIn() {
    viewer?.zoomStep(1);
  }

  function fit() {
    viewer?.fitToView();
  }

  function reset() {
    viewer?.reset();
  }
</script>

<div class="zoom-tools">
  <button class="zoom-btn" on:click={zoomOut} title={t(language, "zoomOut")}>−</button>
  <span class="zoom-badge">{zoomBadgeText}</span>
  <button class="zoom-btn" on:click={zoomIn} title={t(language, "zoomIn")}>+</button>
  <button class="zoom-btn btn-text" on:click={fit} title={t(language, "zoomFit")}>{t(language, "zoomFit")}</button>
  <button class="zoom-btn btn-text" on:click={reset} title={t(language, "zoomReset")}>1:1</button>
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
  }
</style>
