<script lang="ts">
  import Toolbar from "./components/Toolbar.svelte";
  import Dropzone from "./components/Dropzone.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import Canvas from "./components/Canvas.svelte";
  import { convertImage, saveSvgDialog } from "./lib/api";

  let originalDataUri: string | null = null;
  let originalFileName: string = "";
  let originalFilePath: string | undefined = undefined;
  let isPdf: boolean = false;
  let svgContent: string | null = null;

  let mode: "centerline" | "outline" = "centerline";
  let corners: "sharp" | "smooth" = "sharp";
  let threshold: number = 210;
  let autocrop: boolean = true;
  let addFrame: boolean = false;

  let viewMode: "split" | "overlay" = "split";
  let isProcessing: boolean = false;

  let showToast: boolean = false;
  let toastMessage: string = "";
  let toastTimer: any = null;

  function triggerToast(msg: string) {
    toastMessage = msg;
    showToast = true;
    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(() => {
      showToast = false;
    }, 3500);
  }

  function handleFileSelected(data: {
    fileName: string;
    filePath?: string;
    dataUri?: string;
    isPdf: boolean;
  }) {
    originalFileName = data.fileName;
    originalFilePath = data.filePath;
    originalDataUri = data.dataUri || null;
    isPdf = data.isPdf;

    // Otomatik dönüştür
    runConversion();
  }

  async function runConversion() {
    if (!originalDataUri && !originalFilePath) return;

    isProcessing = true;
    try {
      const res = await convertImage({
        mode,
        corners,
        threshold,
        autocrop,
        addFrame,
        base64: originalDataUri || undefined,
        filePath: originalFilePath,
      });

      if (res && res.success && res.svg) {
        svgContent = res.svg;
        triggerToast("✓ Tek Hat SVG Başarıyla Oluşturuldu!");
      }
    } catch (err: any) {
      console.error("Dönüştürme hatası:", err);
      triggerToast(`Hata: ${err?.message || err}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleDownload() {
    if (!svgContent) return;

    const baseName = originalFileName
      ? originalFileName.replace(/\.[^/.]+$/, "")
      : "cizim";
    const suggested = `${baseName}_luban_tek_hat.svg`;

    try {
      const res = await saveSvgDialog(svgContent, suggested);
      if (res.success && res.saved_path) {
        triggerToast("✓ SVG Başarıyla Kaydedildi!");
      }
    } catch (err: any) {
      console.error("Kaydetme hatası:", err);
      // Web tarayıcısı fallback
      const blob = new Blob([svgContent], { type: "image/svg+xml" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = suggested;
      a.click();
      URL.revokeObjectURL(url);
      triggerToast("✓ SVG İndirildi!");
    }
  }

  function handleFolderClick(folder: string) {
    triggerToast(`"${folder}" klasöründen çizim yüklemek için Görsel Seç'i kullanabilirsiniz.`);
  }
</script>

<div class="app-layout">
  <Toolbar
    {viewMode}
    hasSvg={!!svgContent}
    {toastMessage}
    {showToast}
    onViewModeChange={(m) => (viewMode = m)}
    onDownload={handleDownload}
  />

  <div class="main-container">
    <aside class="sidebar-panel">
      <Dropzone
        fileName={originalFileName}
        onFileSelected={handleFileSelected}
      />
      <Sidebar
        bind:mode
        bind:corners
        bind:threshold
        bind:autocrop
        bind:addFrame
        {isProcessing}
        onConvert={runConversion}
        onParamChange={runConversion}
        onFolderClick={handleFolderClick}
      />
    </aside>

    <main class="content-panel">
      <Canvas
        {originalDataUri}
        {originalFileName}
        {svgContent}
        {viewMode}
      />
    </main>
  </div>
</div>

<style>
  :global(:root) {
    --bg: #0f172a;
    --panel: #1e293b;
    --panel-border: #334155;
    --accent: #38bdf8;
    --accent-hover: #0ea5e9;
    --text: #f8fafc;
    --text-muted: #94a3b8;
    --laser-red: #ef4444;
    --success: #10b981;
  }

  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  }

  :global(body) {
    background-color: var(--bg);
    color: var(--text);
    height: 100vh;
    overflow: hidden;
  }

  .app-layout {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }

  .main-container {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .sidebar-panel {
    width: 380px;
    min-width: 380px;
    background: var(--panel);
    border-right: 1px solid var(--panel-border);
    display: flex;
    flex-direction: column;
    padding: 20px;
    gap: 18px;
    overflow-y: auto;
  }

  .content-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: #090d16;
    padding: 16px;
    overflow: hidden;
  }
</style>
