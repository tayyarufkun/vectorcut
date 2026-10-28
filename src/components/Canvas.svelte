<script lang="ts">
  import { onMount, tick } from "svelte";
  import { PanZoomViewer } from "../lib/panzoom";
  import ZoomControls from "./ZoomControls.svelte";
  import { t, type Language } from "../lib/i18n";

  export let language: Language = "en";
  export let originalDataUri: string | null = null;
  export let originalFileName: string = "";
  export let svgContent: string | null = null;
  export let viewMode: "split" | "overlay" = "split";

  let origContainerEl: HTMLElement;
  let origContentEl: HTMLElement;
  let origBadgeText = "100%";
  let origViewer: PanZoomViewer | null = null;

  let svgContainerEl: HTMLElement;
  let svgContentEl: HTMLElement;
  let svgBadgeText = "100%";
  let svgViewer: PanZoomViewer | null = null;

  let overlayContainerEl: HTMLElement;
  let overlayContentEl: HTMLElement;
  let overlayBadgeText = "100%";
  let overlayViewer: PanZoomViewer | null = null;

  function initViewers() {
    if (viewMode === "split") {
      if (origContainerEl && origContentEl && !origViewer) {
        origViewer = new PanZoomViewer(origContainerEl, origContentEl, null, (s) => {
          origBadgeText = `${Math.round(s * 100)}%`;
        });
      }
      if (svgContainerEl && svgContentEl && !svgViewer) {
        svgViewer = new PanZoomViewer(svgContainerEl, svgContentEl, null, (s) => {
          svgBadgeText = `${Math.round(s * 100)}%`;
        });
      }
    } else {
      if (overlayContainerEl && overlayContentEl && !overlayViewer) {
        overlayViewer = new PanZoomViewer(overlayContainerEl, overlayContentEl, null, (s) => {
          overlayBadgeText = `${Math.round(s * 100)}%`;
        });
      }
    }
  }

  onMount(() => {
    initViewers();
    return () => {
      origViewer?.destroy();
      svgViewer?.destroy();
      overlayViewer?.destroy();
    };
  });

  export async function resetViewports() {
    await tick();
    if (viewMode === "split") {
      origViewer?.fitToView();
      svgViewer?.fitToView();
    } else {
      overlayViewer?.fitToView();
    }
  }

  // Mod değiştiğinde
  $: if (viewMode) {
    tick().then(() => {
      initViewers();
      resetViewports();
    });
  }

  // Yeni görsel veya SVG geldiğinde
  $: if (originalDataUri) {
    tick().then(() => {
      setTimeout(() => {
        origViewer?.fitToView();
        overlayViewer?.fitToView();
      }, 50);
    });
  }

  $: if (svgContent) {
    tick().then(() => {
      setTimeout(() => {
        svgViewer?.fitToView();
        overlayViewer?.fitToView();
      }, 50);
    });
  }

  function handleImageLoad() {
    setTimeout(() => {
      origViewer?.fitToView();
      overlayViewer?.fitToView();
    }, 20);
  }
</script>

<div class="canvas-root">
  {#if viewMode === "split"}
    <div class="preview-split">
      <!-- Orijinal Çizim Paneli -->
      <div class="preview-panel">
        <div class="preview-header">
          <div class="header-title">
            <span class="title-bold">{t(language, "originalDrawing")}</span>
            <span class="meta-label">{originalFileName || "-"}</span>
          </div>
          <ZoomControls
            {language}
            zoomBadgeText={origBadgeText}
            onZoomIn={() => origViewer?.zoomStep(1)}
            onZoomOut={() => origViewer?.zoomStep(-1)}
            onFit={() => origViewer?.fitToView()}
            onReset={() => origViewer?.reset()}
          />
        </div>
        <div class="preview-body" bind:this={origContainerEl}>
          <div class="panzoom-content" bind:this={origContentEl}>
            {#if originalDataUri}
              <img
                src={originalDataUri}
                alt={t(language, "originalDrawing")}
                on:load={handleImageLoad}
                draggable="false"
              />
            {:else}
              <div class="placeholder">
                <svg viewBox="0 0 24 24" class="ph-icon">
                  <path d="M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z"/>
                </svg>
                <span>{t(language, "dropPlaceholder")}</span>
              </div>
            {/if}
          </div>
        </div>
      </div>

      <!-- Vektör Çıktısı (SVG) Paneli -->
      <div class="preview-panel">
        <div class="preview-header">
          <div class="header-title">
            <span class="title-bold accent-text">{t(language, "vectorResult")}</span>
            {#if svgContent}
              <span class="status-ready">✓ {t(language, "readyBadge")}</span>
            {/if}
          </div>
          <ZoomControls
            {language}
            zoomBadgeText={svgBadgeText}
            onZoomIn={() => svgViewer?.zoomStep(1)}
            onZoomOut={() => svgViewer?.zoomStep(-1)}
            onFit={() => svgViewer?.fitToView()}
            onReset={() => svgViewer?.reset()}
          />
        </div>
        <div class="preview-body" bind:this={svgContainerEl}>
          <div class="panzoom-content" bind:this={svgContentEl}>
            {#if svgContent}
              <div class="svg-host" draggable="false">
                {@html svgContent}
              </div>
            {:else}
              <div class="placeholder">
                <svg viewBox="0 0 24 24" class="ph-icon">
                  <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
                </svg>
                <span>{t(language, "waitingPlaceholder")}</span>
              </div>
            {/if}
          </div>
        </div>
      </div>
    </div>
  {:else}
    <!-- Overlay Modu (Vektör Hizalama / Katman Bindirme) -->
    <div class="preview-panel preview-overlay">
      <div class="preview-header">
        <div class="header-title">
          <span class="title-bold laser-text">{t(language, "overlayTitle")}</span>
          <span class="meta-label">{t(language, "overlaySubtitle")}</span>
        </div>
        <ZoomControls
          {language}
          zoomBadgeText={overlayBadgeText}
          onZoomIn={() => overlayViewer?.zoomStep(1)}
          onZoomOut={() => overlayViewer?.zoomStep(-1)}
          onFit={() => overlayViewer?.fitToView()}
          onReset={() => overlayViewer?.reset()}
        />
      </div>
      <div class="preview-body" bind:this={overlayContainerEl}>
        <div class="panzoom-content" bind:this={overlayContentEl}>
          <div class="overlay-wrapper">
            {#if originalDataUri}
              <img
                src={originalDataUri}
                alt="Original Layer"
                class="overlay-bg"
                on:load={handleImageLoad}
                draggable="false"
              />
            {/if}
            {#if svgContent}
              <div class="overlay-svg" draggable="false">
                {@html svgContent}
              </div>
            {:else if !originalDataUri}
              <div class="placeholder">
                <span>{t(language, "overlayPlaceholder")}</span>
              </div>
            {/if}
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .canvas-root {
    flex: 1;
    display: flex;
    overflow: hidden;
    position: relative;
  }
  .preview-split {
    flex: 1;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    overflow: hidden;
  }
  .preview-panel {
    background: #ffffff;
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    position: relative;
    overflow: hidden;
    border: 1px solid var(--panel-border);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
  }
  .preview-overlay {
    flex: 1;
  }
  .preview-header {
    background: #f1f5f9;
    color: #334155;
    padding: 8px 14px;
    font-size: 0.8rem;
    border-bottom: 1px solid #e2e8f0;
    display: flex;
    justify-content: space-between;
    align-items: center;
    user-select: none;
  }
  .header-title {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .title-bold {
    font-weight: 700;
  }
  .accent-text {
    color: #0369a1;
  }
  .laser-text {
    color: #b91c1c;
  }
  .meta-label {
    font-size: 0.72rem;
    color: #64748b;
  }
  .status-ready {
    color: #059669;
    font-size: 0.75rem;
    font-weight: 700;
  }
  .preview-body {
    flex: 1;
    position: relative;
    overflow: hidden;
    cursor: grab;
    user-select: none;
    -webkit-user-select: none;
    background-image:
      linear-gradient(45deg, #f8fafc 25%, transparent 25%),
      linear-gradient(-45deg, #f8fafc 25%, transparent 25%),
      linear-gradient(45deg, transparent 75%, #f8fafc 75%),
      linear-gradient(-45deg, transparent 75%, #f8fafc 75%);
    background-size: 20px 20px;
    background-position: 0 0, 0 10px, 10px -10px, -10px 0px;
  }
  :global(.preview-body.dragging) {
    cursor: grabbing !important;
  }
  .panzoom-content {
    position: absolute;
    top: 0;
    left: 0;
    transform-origin: 0 0;
    will-change: transform;
    pointer-events: none;
    user-select: none;
    -webkit-user-select: none;
  }
  .panzoom-content img {
    display: block;
    max-width: none;
    max-height: none;
    pointer-events: none;
    user-select: none;
    -webkit-user-drag: none;
    filter: drop-shadow(0 2px 8px rgba(0, 0, 0, 0.12));
  }
  .svg-host {
    display: block;
    pointer-events: none;
    user-select: none;
  }
  .svg-host :global(svg) {
    display: block;
    max-width: none;
    max-height: none;
    pointer-events: none;
    user-select: none;
    filter: drop-shadow(0 2px 8px rgba(0, 0, 0, 0.12));
  }
  .placeholder {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    color: #94a3b8;
    font-size: 0.85rem;
    text-align: center;
    padding: 20px;
    pointer-events: none;
    user-select: none;
    white-space: nowrap;
  }
  .ph-icon {
    width: 36px;
    height: 36px;
    fill: #cbd5e1;
  }
  .overlay-wrapper {
    position: relative;
    display: block;
    user-select: none;
    pointer-events: none;
  }
  .overlay-bg {
    display: block;
    opacity: 0.45;
    pointer-events: none;
    -webkit-user-drag: none;
  }
  .overlay-svg {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 2;
    pointer-events: none;
  }
  .overlay-svg :global(svg) {
    width: 100%;
    height: 100%;
    display: block;
  }
  .overlay-svg :global(svg path),
  .overlay-svg :global(svg rect) {
    stroke: #ef4444 !important;
    stroke-width: 1.5 !important;
  }
</style>
