<script lang="ts">
  import { onMount } from "svelte";
  import Toolbar from "./components/Toolbar.svelte";
  import Dropzone from "./components/Dropzone.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import Canvas from "./components/Canvas.svelte";
  import PreferencesModal from "./components/PreferencesModal.svelte";
  import LanguageModal from "./components/LanguageModal.svelte";
  import {
    convertImage,
    saveSvgDialog,
    getAiStatus,
    downloadAiModel,
    deleteAiModel,
    onAiDownloadProgress,
    onAiDownloadComplete,
    onAiDownloadError,
    type ModelStatus,
  } from "./lib/api";
  import { loadPreferences, savePreferences, type AppPreferences } from "./lib/preferences";
  import { t, type Language } from "./lib/i18n";

  let preferences: AppPreferences = loadPreferences();
  let showPreferencesModal: boolean = false;
  let showLanguageModal: boolean = false;

  let originalDataUri: string | null = null;
  let originalFileName: string = "";
  let originalFilePath: string | undefined = undefined;
  let isPdf: boolean = false;
  let svgContent: string | null = null;

  let mode: "centerline" | "outline" = "centerline";
  let corners: "sharp" | "smooth" = preferences.defaultCorners || "sharp";
  let threshold: number = preferences.defaultThreshold || 210;
  let autocrop: boolean = preferences.defaultAutoCrop ?? true;
  let addFrame: boolean = preferences.defaultAddFrame ?? false;
  let useAi: boolean = false;

  let aiModelStatus: ModelStatus = { installed: false, path: "", size_mb: 0 };
  let aiDownloading: boolean = false;
  let aiProgress: number = 0;

  let viewMode: "split" | "overlay" = "split";
  let isProcessing: boolean = false;

  let showToast: boolean = false;
  let toastMessage: string = "";
  let toastTimer: any = null;

  onMount(() => {
    preferences = loadPreferences();
    corners = preferences.defaultCorners;
    threshold = preferences.defaultThreshold;
    autocrop = preferences.defaultAutoCrop;
    addFrame = preferences.defaultAddFrame;

    checkAiStatus();

    // Event dinleyicileri
    let unlistenProgress: () => void;
    let unlistenComplete: () => void;
    let unlistenError: () => void;

    onAiDownloadProgress((prog) => {
      aiDownloading = true;
      aiProgress = prog;
    }).then((un) => (unlistenProgress = un));

    onAiDownloadComplete(() => {
      aiDownloading = false;
      aiProgress = 1.0;
      checkAiStatus();
      triggerToast(t(preferences.language, "statusSuccess"));
      if (originalDataUri || originalFilePath) {
        runConversion();
      }
    }).then((un) => (unlistenComplete = un));

    onAiDownloadError((err) => {
      aiDownloading = false;
      aiProgress = 0;
      triggerToast(`AI Download Error: ${err}`);
    }).then((un) => (unlistenError = un));

    return () => {
      if (unlistenProgress) unlistenProgress();
      if (unlistenComplete) unlistenComplete();
      if (unlistenError) unlistenError();
    };
  });

  async function checkAiStatus() {
    try {
      const status = await getAiStatus();
      aiModelStatus = status;
    } catch (e) {
      console.warn("Could not check AI status:", e);
    }
  }

  async function handleDownloadAi() {
    aiDownloading = true;
    aiProgress = 0.0;
    try {
      await downloadAiModel();
    } catch (e: any) {
      aiDownloading = false;
      triggerToast(`AI Error: ${e?.message || e}`);
    }
  }

  async function handleDeleteAi() {
    try {
      await deleteAiModel();
      await checkAiStatus();
      triggerToast(t(preferences.language, "statusSuccess"));
      if (useAi && (originalDataUri || originalFilePath)) {
        runConversion();
      }
    } catch (e: any) {
      triggerToast(`Error: ${e?.message || e}`);
    }
  }

  function triggerToast(msg: string) {
    toastMessage = msg;
    showToast = true;
    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(() => {
      showToast = false;
    }, 3500);
  }

  function handleSavePreferences(newPrefs: AppPreferences) {
    preferences = newPrefs;
    savePreferences(newPrefs);
    triggerToast(t(preferences.language, "prefSavedToast"));
  }

  function handleSelectLanguage(newLang: Language) {
    preferences = { ...preferences, language: newLang };
    savePreferences(preferences);
    triggerToast(t(newLang, "prefSavedToast"));
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
        useAi,
      });

      if (res && res.success && res.svg) {
        svgContent = res.svg;
        triggerToast(t(preferences.language, "statusConverted"));
      }
    } catch (err: any) {
      console.error("Conversion error:", err);
      triggerToast(`Error: ${err?.message || err}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleDownload() {
    if (!svgContent) return;

    const baseName = originalFileName
      ? originalFileName.replace(/\.[^/.]+$/, "")
      : "drawing";

    const suffix =
      mode === "centerline"
        ? preferences.centerlineSuffix || "_centerline"
        : preferences.outlineSuffix || "_outline";

    const suggested = `${baseName}${suffix}.svg`;

    try {
      const res = await saveSvgDialog(svgContent, suggested);
      if (res.success && res.saved_path) {
        triggerToast(t(preferences.language, "statusSaved"));
      }
    } catch (err: any) {
      console.error("Save error:", err);
      // Web fallback
      const blob = new Blob([svgContent], { type: "image/svg+xml" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = suggested;
      a.click();
      URL.revokeObjectURL(url);
      triggerToast(t(preferences.language, "statusSaved"));
    }
  }
</script>

<div class="app-layout">
  <Toolbar
    language={preferences.language}
    {viewMode}
    hasSvg={!!svgContent}
    {toastMessage}
    {showToast}
    onViewModeChange={(m) => (viewMode = m)}
    onDownload={handleDownload}
    onOpenPreferences={() => (showPreferencesModal = true)}
    onOpenLanguage={() => (showLanguageModal = true)}
  />

  <div class="main-container">
    <aside class="sidebar-panel">
      <Dropzone
        language={preferences.language}
        fileName={originalFileName}
        onFileSelected={handleFileSelected}
      />
      <Sidebar
        language={preferences.language}
        bind:mode
        bind:corners
        bind:threshold
        bind:autocrop
        bind:addFrame
        bind:useAi
        aiInstalled={aiModelStatus.installed}
        {aiDownloading}
        {aiProgress}
        {isProcessing}
        onConvert={runConversion}
        onParamChange={runConversion}
        onDownloadAi={handleDownloadAi}
      />
    </aside>

    <main class="content-panel">
      <Canvas
        language={preferences.language}
        {originalDataUri}
        {originalFileName}
        {svgContent}
        {viewMode}
      />
    </main>
  </div>

  <PreferencesModal
    show={showPreferencesModal}
    {preferences}
    aiInstalled={aiModelStatus.installed}
    aiSizeMb={aiModelStatus.size_mb}
    aiPath={aiModelStatus.path}
    {aiDownloading}
    {aiProgress}
    onSave={handleSavePreferences}
    onClose={() => (showPreferencesModal = false)}
    onDownloadAi={handleDownloadAi}
    onDeleteAi={handleDeleteAi}
  />

  <LanguageModal
    show={showLanguageModal}
    currentLanguage={preferences.language}
    onSelectLanguage={handleSelectLanguage}
    onClose={() => (showLanguageModal = false)}
  />
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
    font-family: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }

  :global(body) {
    background-color: var(--bg);
    color: var(--text);
    height: 100vh;
    overflow: hidden;
    font-family: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
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
    width: 360px;
    min-width: 360px;
    background: var(--panel);
    border-right: 1px solid var(--panel-border);
    display: flex;
    flex-direction: column;
    padding: 18px;
    gap: 16px;
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
