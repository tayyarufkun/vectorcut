import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

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

export async function pickFile(): Promise<PickFileResult> {
  return await invoke<PickFileResult>("pick_file");
}

export async function saveSvgDialog(svgContent: string, defaultFilename?: string): Promise<SaveSvgResult> {
  return await invoke<SaveSvgResult>("save_svg_dialog", {
    svgContent,
    defaultFilename,
  });
}

export async function getAiStatus(): Promise<ModelStatus> {
  try {
    return await invoke<ModelStatus>("get_ai_status");
  } catch {
    return { installed: false, path: "", size_mb: 0 };
  }
}

export async function getAiDownloadProgress(): Promise<number> {
  try {
    return await invoke<number>("get_ai_download_progress");
  } catch {
    return 0;
  }
}

export async function downloadAiModel(): Promise<ModelStatus> {
  return await invoke<ModelStatus>("download_ai_model");
}

export async function deleteAiModel(): Promise<void> {
  return await invoke<void>("delete_ai_model");
}

export async function onAiDownloadProgress(callback: (progress: number) => void): Promise<UnlistenFn> {
  try {
    return await listen<number>("ai-model-download-progress", (event) => {
      callback(event.payload);
    });
  } catch {
    return () => {};
  }
}

export async function onAiDownloadComplete(callback: () => void): Promise<UnlistenFn> {
  try {
    return await listen("ai-model-download-complete", () => {
      callback();
    });
  } catch {
    return () => {};
  }
}

export async function onAiDownloadError(callback: (err: string) => void): Promise<UnlistenFn> {
  try {
    return await listen<string>("ai-model-download-error", (event) => {
      callback(event.payload);
    });
  } catch {
    return () => {};
  }
}
