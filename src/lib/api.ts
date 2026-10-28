let tauriInvoke: typeof import("@tauri-apps/api/core").invoke | null = null;

async function getInvoke() {
  if (tauriInvoke) return tauriInvoke;
  try {
    const core = await import("@tauri-apps/api/core");
    tauriInvoke = core.invoke;
    return tauriInvoke;
  } catch (err) {
    console.warn("Tauri ortamı bulunamadı, tarayıcı modunda çalışılıyor:", err);
    return null;
  }
}

export interface ConvertOptions {
  mode: "centerline" | "outline";
  corners: "sharp" | "smooth";
  threshold: number;
  autocrop: boolean;
  addFrame: boolean;
  base64?: string;
  filePath?: string;
  useAi?: boolean;
}

export interface ConvertResult {
  success: boolean;
  svg: string;
  width: number;
  height: number;
}

export interface PickFileResult {
  success: boolean;
  cancelled: boolean;
  filePath?: string;
  fileName?: string;
  isPdf: boolean;
  dataUri?: string;
}

export interface SaveSvgResult {
  success: boolean;
  cancelled: boolean;
  savedPath?: string;
}

export interface ModelStatus {
  installed: boolean;
  path: string;
  size_mb: number;
}

export async function convertImage(options: ConvertOptions): Promise<ConvertResult> {
  const invoke = await getInvoke();
  if (invoke) {
    return await invoke<ConvertResult>("convert_image", {
      request: {
        mode: options.mode,
        corners: options.corners,
        threshold: options.threshold,
        autocrop: options.autocrop,
        add_frame: options.addFrame,
        base64: options.base64,
        file_path: options.filePath,
        use_ai: options.useAi || false,
      },
    });
  }
  throw new Error("Tauri API kullanılamıyor");
}

export async function pickFile(): Promise<PickFileResult> {
  const invoke = await getInvoke();
  if (invoke) {
    return await invoke<PickFileResult>("pick_file");
  }
  throw new Error("Tauri API kullanılamıyor");
}

export async function saveSvgDialog(svgContent: string, defaultFilename?: string): Promise<SaveSvgResult> {
  const invoke = await getInvoke();
  if (invoke) {
    return await invoke<SaveSvgResult>("save_svg_dialog", {
      svgContent,
      defaultFilename,
    });
  }
  throw new Error("Tauri API kullanılamıyor");
}

export async function getAiStatus(): Promise<ModelStatus> {
  const invoke = await getInvoke();
  if (invoke) {
    return await invoke<ModelStatus>("get_ai_status");
  }
  return { installed: false, path: "", size_mb: 0 };
}

export async function downloadAiModel(): Promise<void> {
  const invoke = await getInvoke();
  if (invoke) {
    return await invoke<void>("download_ai_model");
  }
}

export async function deleteAiModel(): Promise<void> {
  const invoke = await getInvoke();
  if (invoke) {
    return await invoke<void>("delete_ai_model");
  }
}

export async function onAiDownloadProgress(callback: (progress: number) => void): Promise<() => void> {
  try {
    const { listen } = await import("@tauri-apps/api/event");
    return await listen<number>("ai-model-download-progress", (event) => {
      callback(event.payload);
    });
  } catch {
    return () => {};
  }
}

export async function onAiDownloadComplete(callback: () => void): Promise<() => void> {
  try {
    const { listen } = await import("@tauri-apps/api/event");
    return await listen("ai-model-download-complete", () => {
      callback();
    });
  } catch {
    return () => {};
  }
}

export async function onAiDownloadError(callback: (err: string) => void): Promise<() => void> {
  try {
    const { listen } = await import("@tauri-apps/api/event");
    return await listen<string>("ai-model-download-error", (event) => {
      callback(event.payload);
    });
  } catch {
    return () => {};
  }
}
