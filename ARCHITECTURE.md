# VectorCut Studio — Technical Architecture & Specifications

This document outlines the modern, cross-platform architecture of **VectorCut Studio**, built with **Tauri v2 (Rust)** and **Svelte 5 (Vite)**.

---

## 1. System Architecture

```text
┌────────────────────────────────────────────────────────────────────────┐
│                   TAURI SHELL (macOS, Windows, Linux)                  │
├───────────────────────────────────┬────────────────────────────────────┤
│      FRONTEND (Svelte 5 + Vite)   │         BACKEND (Rust Core)        │
│                                   │                                    │
│  • src/components/                │  • src-tauri/src/                  │
│    ├── Canvas.svelte (Pan & Zoom) │    ├── main.rs (IPC & native OS)   │
│    ├── Dropzone.svelte (Input)    │    ├── image_proc.rs (Binarize)    │
│    ├── Sidebar.svelte (Params)    │    ├── tracer.rs (Thinning & RDP)  │
│    ├── Toolbar.svelte (Actions)   │    ├── pdf.rs (Vector parser)      │
│    ├── PreferencesModal.svelte    │    └── autocrop.rs (Bounding box)  │
│    └── LanguageModal.svelte       │                                    │
│  • src/lib/panzoom.ts             │  Zero external binaries            │
│  • src/lib/i18n.ts                │  Blazing-fast native speed         │
└───────────────────────────────────┴────────────────────────────────────┘
                                    │
                        GitHub Actions Workflows
                ┌───────────────────┼───────────────────┐
                ▼                   ▼                   ▼
           macOS (.dmg)       Windows (.exe)       Linux (.AppImage)
```

---

## 2. Core Modules

### A. Rust Core (`src-tauri/`)
1. **`image_proc.rs`**:  
   Performs alpha blending over white backgrounds, calculates perceptual luminance, and binarizes the raster drawing according to user threshold sensitivity.
2. **`autocrop.rs`**:  
   Extracts strict pixel bounding box coordinates (`min_x, min_y, max_x, max_y`) and eliminates blank margins with padding.
3. **`tracer.rs`**:  
   * **Zhang-Suen Thinning Algorithm**: Iteratively erodes contour pixels to derive a 1-pixel medial axis skeleton.
   * **Graph & Path Extraction**: Traverses endpoints, junctions, and closed cycles into discrete polylines.
   * **Ramer-Douglas-Peucker (RDP)**: 
     * `sharp`: Preserves true 90-degree orthogonal lines.
     * `smooth`: Transforms organic curvature into smooth Bezier paths.
   * Formats into compliant SVG paths (`fill="none" stroke="#000000" stroke-width="0.5"`).
4. **`pdf.rs`**:  
   Parses vector PDF stream tokens (`m`, `l`, `c`, `re`, `S`) directly into lossless SVG paths without raster sampling loss.
5. **`main.rs`**:  
   Handles cross-platform native file selection and "Save As" file pickers via `rfd`.

### B. Svelte Frontend (`src/`)
1. **`Canvas.svelte` & `panzoom.ts`**:  
   Hardware-accelerated Pan & Zoom with persistent viewport coordinates during parameter tweaks.
2. **`Sidebar.svelte`**:  
   Single-line vs. outline modes, sharp vs. smooth geometry, binarization threshold, auto-crop, and perimeter framing.
3. **`PreferencesModal.svelte` & `LanguageModal.svelte`**:  
   Persistent settings stored in `localStorage` for export suffixes and interface languages (English / Turkish).
