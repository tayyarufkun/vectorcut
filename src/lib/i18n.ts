export type Language = "en" | "tr";

export const translations = {
  en: {
    // Toolbar & Brand
    appTitle: "VectorCut Studio",
    appSubtitle: "Universal Vector & SVG Converter",
    viewSplit: "Side by Side",
    viewOverlay: "Overlay (Trace Alignment)",
    exportSvg: "Export SVG",
    preferences: "Preferences",
    languageSelect: "Language",
    languageModalTitle: "Select Interface Language",
    statusSuccess: "Operation Successful",
    statusConverted: "SVG Generated Successfully",
    statusSaved: "SVG Saved Successfully",

    // Dropzone
    dropzoneTitle: "Image / PDF Input",
    dropzonePrompt: "Click or Drop Drawing Here",
    dropzoneSub: "Supports PNG, JPG, JPEG, and Vector PDF",

    // Sidebar & Parameters
    parametersTitle: "Conversion Parameters",
    modeLabel: "Trace Mode:",
    modeCenterline: "Single Line",
    modeCenterlineSub: "Centerline (Strokes)",
    modeOutline: "Outer Contour",
    modeOutlineSub: "Outline (Silhouettes)",
    cornerLabel: "Corner Geometry:",
    cornerSharp: "Sharp Corners",
    cornerSharpSub: "Preserves 90° lines, boxes, joints",
    cornerSmooth: "Smooth Curves",
    cornerSmoothSub: "Organic curves and arcs",
    thresholdLabel: "B&W Sensitivity (Threshold):",
    autoCropLabel: "Auto-Crop (Tight Bounds)",
    autoCropHint: "Trims empty outer margins and snaps SVG bounds to the drawing.",
    addFrameLabel: "Add Perimeter Frame",
    addFrameHint: "Adds an outer rectangular boundary frame around the design.",
    btnGenerate: "Generate SVG Path",
    btnGenerating: "Processing...",

    // Canvas
    originalDrawing: "ORIGINAL DRAWING",
    vectorResult: "VECTOR OUTPUT (SVG)",
    readyBadge: "Ready to Use",
    waitingPlaceholder: "Waiting for conversion...",
    dropPlaceholder: "Select or drop an image / PDF from the left panel",
    overlayTitle: "TRACE ALIGNMENT (OVERLAY VIEW)",
    overlaySubtitle: "Red line: Vector path | Faded background: Original image",
    overlayPlaceholder: "Vector overlay preview will appear once an image is loaded",

    // Zoom
    zoomOut: "Zoom Out",
    zoomIn: "Zoom In",
    zoomFit: "Fit View",
    zoomReset: "1:1 Reset",

    // Preferences Modal
    prefTitle: "Preferences",
    prefGeneral: "General Settings",
    prefLanguage: "Interface Language",
    prefFileNaming: "Export File Suffixes",
    prefCenterlineSuffix: "Centerline Suffix",
    prefCenterlineSuffixHint: "Appended to filename in Single-Line mode (e.g. '_centerline')",
    prefOutlineSuffix: "Outline Suffix",
    prefOutlineSuffixHint: "Appended to filename in Outline mode (e.g. '_outline')",
    prefDefaults: "Default Conversion Options",
    prefDefaultCorner: "Default Corner Style",
    prefReset: "Reset Defaults",
    prefSave: "Save & Close",
    prefSavedToast: "Preferences saved successfully",
  },
  tr: {
    // Toolbar & Brand
    appTitle: "VectorCut Studio",
    appSubtitle: "Evrensel Vektör & SVG Dönüştürücü",
    viewSplit: "Yan Yana",
    viewOverlay: "Overlay (Çizim & Vektör Hizalama)",
    exportSvg: "SVG Dışa Aktar",
    preferences: "Tercihler",
    languageSelect: "Dil",
    languageModalTitle: "Arayüz Dili Seçimi",
    statusSuccess: "İşlem Başarılı",
    statusConverted: "SVG Başarıyla Oluşturuldu",
    statusSaved: "SVG Başarıyla Kaydedildi",

    // Dropzone
    dropzoneTitle: "Görsel / PDF Girişi",
    dropzonePrompt: "Görsel veya PDF Seçmek İçin Tıklayın",
    dropzoneSub: "PNG, JPG, JPEG ve Vektörel PDF Destekler",

    // Sidebar & Parameters
    parametersTitle: "Dönüştürme Parametreleri",
    modeLabel: "Çizgi Modu:",
    modeCenterline: "Tek Hat",
    modeCenterlineSub: "Centerline (Çizgiler)",
    modeOutline: "Dış Kontur",
    modeOutlineSub: "Outline (Silüetler)",
    cornerLabel: "Köşe Yapısı:",
    cornerSharp: "Dik Köşeler",
    cornerSharpSub: "90° dik hatlar, kutu, çerçeve",
    cornerSmooth: "Eğrisel / Yay",
    cornerSmoothSub: "Organik yaylar & eğriler",
    thresholdLabel: "Siyah/Beyaz Hassasiyeti:",
    autoCropLabel: "Boşlukları Kırp (Sıkı Çerçeve)",
    autoCropHint: "Gereksiz dış boşlukları atar, SVG sınırını çizime sıfırlar.",
    addFrameLabel: "Dış Çevre Çerçevesi Ekle",
    addFrameHint: "Çizimin en dış sınırlarını çevreleyen bir dikdörtgen ekler.",
    btnGenerate: "Vektör SVG Oluştur",
    btnGenerating: "Dönüştürülüyor...",

    // Canvas
    originalDrawing: "ORİJİNAL ÇİZİM",
    vectorResult: "VEKTÖR ÇIKTISI (SVG)",
    readyBadge: "Kullanıma Hazır",
    waitingPlaceholder: "Dönüştürme bekleniyor...",
    dropPlaceholder: "Soldaki alandan bir görsel veya PDF seçin",
    overlayTitle: "VEKTÖR HİZALAMA (KATMAN BİNDİRME)",
    overlaySubtitle: "Kırmızı hat: Vektör çizgisi | Soluk arka plan: Orijinal çizim",
    overlayPlaceholder: "Görsel seçildiğinde vektör yolu burada üst üste bindirilecek",

    // Zoom
    zoomOut: "Uzaklaştır",
    zoomIn: "Yakınlaştır",
    zoomFit: "Sığdır",
    zoomReset: "1:1 Sıfırla",

    // Preferences Modal
    prefTitle: "Tercihler",
    prefGeneral: "Genel Ayarlar",
    prefLanguage: "Arayüz Dili",
    prefFileNaming: "Dışa Aktarılan Dosya Ekleri",
    prefCenterlineSuffix: "Tek Hat (Centerline) Dosya Eki",
    prefCenterlineSuffixHint: "Tek hat modunda dosya adına eklenir (örn: '_centerline')",
    prefOutlineSuffix: "Dış Kontur (Outline) Dosya Eki",
    prefOutlineSuffixHint: "Kontur modunda dosya adına eklenir (örn: '_outline')",
    prefDefaults: "Varsayılan Parametreler",
    prefDefaultCorner: "Varsayılan Köşe Yapısı",
    prefReset: "Varsayılana Sıfırla",
    prefSave: "Kaydet & Kapat",
    prefSavedToast: "Tercihler başarıyla kaydedildi",
  },
};

export type TranslationKey = keyof typeof translations.en;

export function t(lang: Language, key: TranslationKey): string {
  const dict = translations[lang] || translations.en;
  return dict[key] || translations.en[key] || key;
}
