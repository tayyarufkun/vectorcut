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

export async function saveSvgDialog(svgContent: String, defaultFilename?: string): Promise<SaveSvgResult> {
  const invoke = await getInvoke();
  if (invoke) {
    return await invoke<SaveSvgResult>("save_svg_dialog", {
      svgContent,
      defaultFilename,
    });
  }
  throw new Error("Tauri API kullanılamıyor");
}

export async function listQuickFolders(): Promise<string[]> {
  const invoke = await getInvoke();
  if (invoke) {
    return await invoke<string[]>("list_quick_folders");
  }
  return ["Teleferik", "Vosvos", "Kauçuk Baskı"];
}
