# VectorCut (svgapp) — Mimari ve Yeniden Yapılandırma Yol Haritası

Bu belge, uygulamanın tek kaynaklı (monolitik) prototip halinden **Tauri (Rust) + Svelte (Vite)** mimarisiyle modern, modüler ve çapraz platformlu (macOS, Windows, Linux) profesyonel bir açık kaynak masaüstü uygulamasına dönüştürülme planını içerir.

---

## 1. Mimari Genel Bakış

```
┌────────────────────────────────────────────────────────────────────────┐
│                   TAURI MASAÜSTÜ (macOS, Windows, Linux)              │
├───────────────────────────────────┬────────────────────────────────────┤
│      FRONTEND (Svelte + Vite)     │         BACKEND (Rust Core)        │
│                                   │                                    │
│  • src/components/                │  • src-tauri/src/                  │
│    ├── Dropzone.svelte (Dosya)    │    ├── main.rs (Pencere & IPC)     │
│    ├── Sidebar.svelte (Ayarlar)   │    ├── image_proc.rs (Eşikleme)    │
│    ├── Canvas.svelte (Pan & Zoom) │    ├── tracer.rs (Centerline & Cut)│
│    └── Toolbar.svelte (Kaydetme)  │    ├── pdf_parser.rs (Kayıpsız PDF)│
│  • src/lib/panzoom.ts             │    └── autocrop.rs (Sıkı Çerçeve)  │
│                                   │                                    │
│  Tüy gibi hafif, 0 KB runtime      │  Sıfır Kurulum, %100 Bağımsız      │
└───────────────────────────────────┴────────────────────────────────────┘
                                    │
                         Otomatik GitHub Actions
                ┌───────────────────┼───────────────────┐
                ▼                   ▼                   ▼
           macOS (.dmg)       Windows (.exe)       Linux (.AppImage)
```

---

## 2. Dizin Yapısı (Modüler Tasarım)

```text
app/
├── .github/
│   └── workflows/
│       └── release.yml          # Otomatik Mac (.dmg) ve Windows (.exe) derleme
├── src/                         # Svelte Frontend
│   ├── assets/
│   │   └── logo.svg
│   ├── components/
│   │   ├── Dropzone.svelte      # Dosya sürükle-bırak ve Finder seçimi
│   │   ├── Sidebar.svelte       # Dik/Yay köşe, eşikleme, çerçeve ayarları
│   │   ├── Canvas.svelte        # Canlı lazer simülasyonu ve önizleme
│   │   ├── ZoomControls.svelte  # Yakınlaştırma, uzaklaştırma, sığdır butonları
│   │   └── Toolbar.svelte       # Dışa aktarma ve Luban SVG kaydetme
│   ├── lib/
│   │   ├── panzoom.ts           # Mouse ile kaydırma & tekerlekle zoom motoru
│   │   └── api.ts               # Rust arka planı ile iletişim (Tauri IPC)
│   ├── App.svelte               # Ana uygulama çatısı
│   └── main.ts                  # Frontend başlangıç noktası
├── src-tauri/                   # Rust Backend (Sıfır Kurulum Çekirdeği)
│   ├── Cargo.toml               # Rust bağımlılıkları (image, resvg, tauri)
│   ├── tauri.conf.json          # Uygulama pencere ve kimlik yapılandırması
│   ├── icons/                   # Uygulama ikonları (.icns, .ico, .png)
│   └── src/
│       ├── main.rs              # Uygulama başlangıcı ve Tauri komutları
│       ├── image_proc.rs        # Görüntü filtreleme, alfa temizleme, binarizasyon
│       ├── autocrop.rs          # Çizimin dışındaki boş alanları sıkı kırpma
│       ├── tracer.rs            # Tek hat (centerline) ve dış kontur algoritması
│       └── pdf.rs               # Vektörel PDF dosyalarını kayıpsız SVG'ye çevirme
├── package.json                 # Frontend bağımlılıkları (Svelte, Vite, TypeScript)
├── vite.config.ts               # Vite yapılandırması
├── README.md                    # Proje tanıtım ve kurulum dokümanı
└── LICENSE                      # MIT Açık Kaynak Lisansı
```

---

## 3. Temel Bileşenler ve Görevleri

### A. Rust Çekirdeği (Backend)
1. **`image_proc.rs`**:  
   Kullanıcının yüklediği görseli (`PNG`, `JPG`) alır, şeffaflığı temizler, kontrastı artırır ve kullanıcının seçtiği eşik değerine (Threshold) göre saf siyah-beyaza dönüştürür.
2. **`autocrop.rs`**:  
   Çizimin piksellerini tarayarak en dış koordinatları (`min_x, min_y, max_x, max_y`) tespit eder. Dışarıdaki gereksiz boşlukları kırpar.
3. **`tracer.rs`**:  
   Piksellerin ortasından tek bir hat (centerline) geçirir. Köşe parametrelerine göre (`sharp` veya `smooth`):
   * `sharp`: 90 derece dik köşeleri korur (`L` hatları).
   * `smooth`: Organik yayları Bezier eğrilerine (`C`) çevirir.
   Çıkan çizgilere `fill="none" stroke="#000000" stroke-width="0.5"` stillerini ekler.
4. **`pdf.rs`**:  
   Vektörel PDF dosyalarını piksellestirmeden doğrudan matematiksel SVG yollarına dönüştürür.

### B. Svelte Arayüzü (Frontend)
1. **`Canvas.svelte` & `panzoom.ts`**:  
   Farenin sol tuşuyla tut-kaydır (Pan) ve tekerlekle %3000'e kadar yakınlaş (Zoom) desteği. Yan yana ve üst üste bindirme (overlay) modları.
2. **`Sidebar.svelte`**:  
   * Tek Hat (Centerline) / Dış Kontur (Outline) modu
   * Dik Köşeler / Yay modu
   * Boşlukları Kırp (Sıkı Çerçeve) / Dış Kesim Çerçevesi Ekle seçenekleri
   * Eşik değeri kaydırıcısı
3. **`Toolbar.svelte`**:  
   Doğrudan işletim sisteminin yerel **"Farklı Kaydet (Save As)"** penceresini tetikler.

---

## 4. GitHub Dağıtım ve Yayınlama (CI/CD)

GitHub Actions iş akışı (`release.yml`), her yeni sürüm etiketinde (örn: `v1.0.0`) şunları otomatik üretir:
* **macOS:** `VectorCut-x64.dmg` ve `VectorCut-aarch64.dmg` (Apple Silicon M1/M2/M3/M4)
* **Windows:** `VectorCut-x64-setup.exe` ve `VectorCut-x64.msi`
* **Linux:** `VectorCut-amd64.AppImage` ve `.deb`

Kullanıcılar terminal açmadan, tek tıkla indirip çift tıklayarak kullanabilir.
