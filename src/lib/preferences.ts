export interface AppPreferences {
  language: "en" | "tr";
  centerlineSuffix: string;
  outlineSuffix: string;
  defaultCorners: "sharp" | "smooth";
  defaultThreshold: number;
  defaultAutoCrop: boolean;
  defaultAddFrame: boolean;
}

const STORAGE_KEY = "vectorcut_preferences_v1";

export const defaultPreferences: AppPreferences = {
  language: "en",
  centerlineSuffix: "_centerline",
  outlineSuffix: "_outline",
  defaultCorners: "sharp",
  defaultThreshold: 210,
  defaultAutoCrop: true,
  defaultAddFrame: false,
};

export function loadPreferences(): AppPreferences {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) {
      const parsed = JSON.parse(raw);
      return { ...defaultPreferences, ...parsed };
    }
  } catch (e) {
    console.warn("Preferences could not be loaded from localStorage:", e);
  }
  return { ...defaultPreferences };
}

export function savePreferences(prefs: AppPreferences): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(prefs));
  } catch (e) {
    console.error("Preferences could not be saved to localStorage:", e);
  }
}
