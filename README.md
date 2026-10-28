# ⚡️ VectorCut Studio

> **Transform raster drawings (PNG, JPG, PDF) into clean, single-line (centerline) cutting paths for laser cutters.**  
> *Lazer kesiciler için çizimleri çift hat yapmadan tek çizgi kesim yoluna (SVG) dönüştüren hafif ve modern masaüstü stüdyosu.*

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey.svg)]()
[![Built with](https://img.shields.io/badge/Stack-Tauri%20%2B%20Svelte%20%2B%20Rust-orange.svg)]()

---

## 🎯 The Problem / Çözülen Problem

* **The Issue:** When you draw lines in Procreate, Affinity Designer, Photoshop, or scan hand-drawn sketches and export as PNG, standard laser software traces the **outer perimeter of each line (double line / outline)** or engraves it as raster (B&W). A 1-line rectangle ends up cutting twice, ruining precision woodwork, acrylic models, and joints!
* **The Solution:** VectorCut uses a dedicated centerline skeletonization engine to extract the **exact single center line** of your strokes, guaranteeing a single laser pass, sharp 90° corners, and zero empty margins.

---

## ✨ Features / Özellikler

* **Single-Line Laser Cut (Centerline Trace):** Traces the true center of black lines instead of wrapping around both sides (No double lines).
* **Sharp Corners vs. Smooth Curves:**
  * 📐 **Sharp (Dik Köşeler):** Preserves crisp 90-degree corners for boxes, joints, slots, windows, and mechanical parts.
  * 🌀 **Smooth (Eğrisel / Yay):** Retains organic bezier curves for artistic drawings and curved vehicles.
* **Auto-Crop (Sıkı Çerçeve):** Removes all empty canvas whitespace, setting the SVG frame directly onto the outermost points of your geometry.
* **Add Outer Cut Frame (Dış Kesim Çerçevesi):** Optionally generates a cutting perimeter rectangle around freestanding designs.
* **Interactive Canvas (Pan & Zoom):**
  * Click and drag to pan freely.
  * Mouse wheel / trackpad pinch zoom up to 3000%.
  * Side-by-side comparison and **Laser Overlay Mode** (red laser beam path superimposed on the original drawing).
* **Direct PDF Support:** Losslessly converts vector PDF blueprints into Luban-ready SVGs with zero pixel distortion.
* **Native OS Save Dialog:** Uses macOS / Windows native "Save As" file picker.
* **Ultra Lightweight:** Uses native OS WebKit/WebView2 (~10 MB binary, ~30 MB RAM) instead of heavy Chromium/Electron.

---

## 🔌 Laser Software Compatibility / Lazer Programı Uyumluluğu

| Laser Software | Vector Cut (Single Line) | Fill / Engrave | Auto-Framing |
| :--- | :---: | :---: | :---: |
| **Snapmaker Luban** | ✅ %100 Uyumlu | ✅ | ✅ |
| **LightBurn** | ✅ %100 Uyumlu | ✅ | ✅ |
| **LaserGRBL** | ✅ %100 Uyumlu | ✅ | ✅ |
| **xTool Creative Space** | ✅ %100 Uyumlu | ✅ | ✅ |
| **Inkscape / Illustrator** | ✅ %100 Uyumlu | ✅ | ✅ |

---

## 🚀 Quickstart / Hızlı Başlangıç

### macOS
1. `Releases` sayfasından en son `.dmg` dosyasını indirin.
2. `VectorCut.app` dosyasını `Applications` (Uygulamalar) klasörüne sürükleyin.
3. Çift tıklayarak başlatın!

### Windows
1. `Releases` sayfasından `.msi` veya `.exe` kurulum dosyasını indirin.
2. Kurulumu tamamlayıp masaüstü simgesinden başlatın.

---

## 🛠️ Architecture / Mimari

* **Desktop Shell & Backend:** [Tauri](https://tauri.app/) (Rust)
* **Frontend UI:** [Svelte](https://svelte.dev/) + [Vite](https://vitejs.dev/) + TypeScript
* **Vectorizing Core:** Pure Rust Image Processing & Centerline Skeletonization
* **Distribution:** Automated multi-platform builds via GitHub Actions (`.github/workflows/release.yml`)

Detaylı teknik mimari için [ARCHITECTURE.md](ARCHITECTURE.md) dosyasını inceleyebilirsiniz.

---

## 📄 License / Lisans

Bu proje [MIT Lisansı](LICENSE) altında açık kaynak olarak sunulmaktadır. Ticari ve kişisel amaçlarla özgürce kullanılabilir, değiştirilebilir ve dağıtılabilir.
