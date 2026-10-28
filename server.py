#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
svgapp - Yerel Web Sunucusu
Snapmaker Luban ve Lazer Kesim için PNG/JPG/PDF -> SVG Dönüştürücü Backend
"""

import os
import sys
import json
import base64
import re
import urllib.parse
from http.server import HTTPServer, BaseHTTPRequestHandler
from pathlib import Path
import subprocess

try:
    from PIL import Image, ImageOps
except ImportError:
    print("Hata: Pillow kütüphanesi eksik!")
    sys.exit(1)

# Dosya yolları
APP_DIR = Path(__file__).resolve().parent
BASE_DIR = Path(os.path.expanduser("~/Desktop/Lazer"))
PORT = 8765

def process_pdf_to_svg(pdf_bytes):
    tmp_pdf = APP_DIR / ".tmp_input.pdf"
    tmp_svg = APP_DIR / ".tmp_output.svg"
    try:
        with open(tmp_pdf, "wb") as f:
            f.write(pdf_bytes)
            
        cmd = [
            "/opt/homebrew/bin/inkscape",
            str(tmp_pdf),
            "--export-plain-svg",
            "--export-type=svg",
            "--export-filename=" + str(tmp_svg)
        ]
        subprocess.run(cmd, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        
        with open(tmp_svg, "r", encoding="utf-8") as f:
            svg_content = f.read()
            
        if "viewBox" not in svg_content:
            dim_match = re.search(r'<svg[^>]*width="([\d.]+)"[^>]*height="([\d.]+)"', svg_content)
            if dim_match:
                w, h = dim_match.group(1), dim_match.group(2)
                svg_content = re.sub(r'<svg', f'<svg viewBox="0 0 {w} {h}"', svg_content, count=1)

        svg_content = re.sub(
            r'style="[^"]*"',
            'style="fill:none;stroke:#000000;stroke-width:0.5;"',
            svg_content
        )
        return svg_content
    finally:
        if tmp_pdf.exists():
            tmp_pdf.unlink()
        if tmp_svg.exists():
            tmp_svg.unlink()

def process_image_to_svg(image_bytes, mode="centerline", corners="sharp", threshold=210, autocrop=True, add_frame=False):
    from io import BytesIO
    
    # PDF kontrolü
    if image_bytes.startswith(b"%PDF"):
        try:
            return process_pdf_to_svg(image_bytes)
        except Exception as e:
            print(f"Vektör PDF çevrim hatası: {e}")

    im = Image.open(BytesIO(image_bytes)).convert("RGBA")
    
    # 1. Şeffaflığı beyaz yap
    bg = Image.new("RGBA", im.size, (255, 255, 255, 255))
    bg.paste(im, mask=im.split()[3])
    gray = bg.convert("L")
    
    # 2. Saf siyah-beyaz binarize
    binary = gray.point(lambda p: 255 if p > threshold else 0, mode='1')
    
    # 3. Otomatik Kırpma (Boş dış alanı at, şeklin en dış sınırlarına sıfırla)
    if autocrop:
        inv = ImageOps.invert(binary.convert("L"))
        bbox = inv.getbbox()
        if bbox:
            pad = 2
            x0 = max(0, bbox[0] - pad)
            y0 = max(0, bbox[1] - pad)
            x1 = min(binary.width, bbox[2] + pad)
            y1 = min(binary.height, bbox[3] + pad)
            binary = binary.crop((x0, y0, x1, y1))

    tmp_bmp = APP_DIR / ".tmp_preview.bmp"
    tmp_svg = APP_DIR / ".tmp_preview.svg"
    binary.save(tmp_bmp)
    
    cmd = [
        "/opt/homebrew/bin/autotrace",
        "--color-count", "2",
        "--output-format", "svg"
    ]
    
    if mode == "centerline":
        cmd.append("--centerline")
        
    if corners == "sharp":
        cmd.extend([
            "--filter-iterations", "0",
            "--corner-threshold", "120",
            "--corner-always-threshold", "70",
            "--line-threshold", "5",
            "--line-reversion-threshold", "5",
            "--error-threshold", "1"
        ])
    else:
        cmd.extend([
            "--filter-iterations", "4",
            "--corner-threshold", "60",
            "--error-threshold", "2"
        ])
        
    cmd.extend(["--output-file", str(tmp_svg), str(tmp_bmp)])
    
    try:
        subprocess.run(cmd, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        with open(tmp_svg, "r", encoding="utf-8") as f:
            svg_content = f.read()
            
        # viewBox ekle
        dim_match = re.search(r'<svg[^>]*width="([\d.]+)"[^>]*height="([\d.]+)"', svg_content)
        w, h = 0, 0
        if dim_match:
            w, h = float(dim_match.group(1)), float(dim_match.group(2))
            if "viewBox" not in svg_content:
                svg_content = re.sub(r'<svg', f'<svg viewBox="0 0 {w} {h}"', svg_content, count=1)

        # Dış Kesim Çerçevesi Ekle (Eğer seçilmişse)
        if add_frame and w > 0 and h > 0:
            frame_rect = f'<rect x="0.5" y="0.5" width="{w-1}" height="{h-1}" style="fill:none;stroke:#000000;stroke-width:0.5;" />'
            svg_content = svg_content.replace('</svg>', f'{frame_rect}</svg>')

        # Luban uyumlu stil ekle
        svg_content = re.sub(
            r'style="[^"]*"',
            'style="fill:none;stroke:#000000;stroke-width:0.5;"',
            svg_content
        )
        return svg_content
    finally:
        if tmp_bmp.exists():
            tmp_bmp.unlink()
        if tmp_svg.exists():
            tmp_svg.unlink()

class LaserAppHandler(BaseHTTPRequestHandler):
    def end_headers_with_cors(self, content_type="application/json"):
        self.send_header("Content-Type", content_type)
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.end_headers()

    def do_OPTIONS(self):
        self.send_response(200)
        self.end_headers_with_cors()

    def do_GET(self):
        parsed = urllib.parse.urlparse(self.path)
        path = parsed.path
        
        if path in ["/", "/index.html"]:
            html_file = APP_DIR / "index.html"
            if html_file.exists():
                with open(html_file, "rb") as f:
                    content = f.read()
                self.send_response(200)
                self.end_headers_with_cors("text/html; charset=utf-8")
                self.wfile.write(content)
            else:
                self.send_response(404)
                self.end_headers()
            return

        if path == "/api/pick-file":
            try:
                cmd = ["osascript", "-e", 'POSIX path of (choose file of type {"public.image", "png", "jpg", "jpeg", "PNG", "JPG", "JPEG", "pdf", "PDF"} with prompt "Lazer için görsel veya PDF seçin:")']
                res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                if res.returncode == 0 and res.stdout.strip():
                    chosen = res.stdout.strip()
                    file_path = Path(chosen)
                    with open(file_path, "rb") as f:
                        data_bytes = f.read()
                    b64 = base64.b64encode(data_bytes).decode("utf-8")
                    
                    is_pdf = file_path.suffix.lower() == ".pdf"
                    mime = "application/pdf" if is_pdf else ("image/png" if file_path.suffix.lower() == ".png" else "image/jpeg")
                    data_uri = f"data:{mime};base64,{b64}"

                    self.send_response(200)
                    self.end_headers_with_cors()
                    self.wfile.write(json.dumps({
                        "success": True,
                        "filePath": chosen,
                        "fileName": file_path.name,
                        "isPdf": is_pdf,
                        "dataUri": data_uri
                    }).encode("utf-8"))
                    return
                else:
                    self.send_response(200)
                    self.end_headers_with_cors()
                    self.wfile.write(json.dumps({"success": False, "cancelled": True}).encode("utf-8"))
                    return
            except Exception as e:
                self.send_response(500)
                self.end_headers_with_cors()
                self.wfile.write(json.dumps({"success": False, "error": str(e)}).encode("utf-8"))
            return
            
        if path == "/api/list-folders":
            if BASE_DIR.exists():
                folders = [d.name for d in BASE_DIR.iterdir() if d.is_dir() and not d.name.startswith(".") and d.name != "app" and not d.name.endswith(".app")]
            else:
                folders = []
            self.send_response(200)
            self.end_headers_with_cors()
            self.wfile.write(json.dumps({"folders": sorted(folders)}).encode("utf-8"))
            return
            
        if path == "/api/list-images":
            query = urllib.parse.parse_qs(parsed.query)
            folder_name = query.get("folder", [""])[0]
            target_dir = BASE_DIR / folder_name
            if not target_dir.exists() or not target_dir.is_dir():
                self.send_response(404)
                self.end_headers_with_cors()
                self.wfile.write(json.dumps({"error": "Klasör bulunamadı"}).encode("utf-8"))
                return
                
            imgs = []
            for ext in ["*.png", "*.jpg", "*.jpeg", "*.PNG", "*.JPG", "*.JPEG", "*.pdf", "*.PDF"]:
                imgs.extend(target_dir.glob(ext))
            imgs = [img.name for img in imgs if not img.name.startswith(".tmp")]
            
            self.send_response(200)
            self.end_headers_with_cors()
            self.wfile.write(json.dumps({"images": sorted(imgs), "folder": folder_name}).encode("utf-8"))
            return

        if path == "/api/get-image":
            query = urllib.parse.parse_qs(parsed.query)
            rel_path = query.get("path", [""])[0]
            file_path = Path(rel_path) if Path(rel_path).is_absolute() else (BASE_DIR / rel_path)
            if file_path.exists() and file_path.is_file():
                with open(file_path, "rb") as f:
                    content = f.read()
                if file_path.suffix.lower() == ".pdf":
                    mime = "application/pdf"
                elif file_path.suffix.lower() == ".png":
                    mime = "image/png"
                else:
                    mime = "image/jpeg"
                self.send_response(200)
                self.end_headers_with_cors(mime)
                self.wfile.write(content)
                return
            self.send_response(404)
            self.end_headers()
            return

        self.send_response(404)
        self.end_headers()

    def do_POST(self):
        parsed = urllib.parse.urlparse(self.path)
        content_length = int(self.headers.get("Content-Length", 0))
        post_data = self.rfile.read(content_length)
        
        if parsed.path == "/api/save-svg":
            try:
                data = json.loads(post_data.decode("utf-8"))
                svg_content = data.get("svg", "")
                suggested_name = data.get("filename", "luban_tek_hat.svg")
                if not suggested_name.lower().endswith(".svg"):
                    suggested_name += ".svg"
                    
                script = f'POSIX path of (choose file name with prompt "Luban SVG Dosyasını Kaydet:" default name "{suggested_name}" default location (path to downloads folder))'
                cmd = ["osascript", "-e", script]
                res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                
                if res.returncode == 0 and res.stdout.strip():
                    dest_path = Path(res.stdout.strip())
                    if not dest_path.name.lower().endswith(".svg"):
                        dest_path = dest_path.with_suffix(".svg")
                    with open(dest_path, "w", encoding="utf-8") as f:
                        f.write(svg_content)
                    self.send_response(200)
                    self.end_headers_with_cors()
                    self.wfile.write(json.dumps({"success": True, "savedPath": str(dest_path)}).encode("utf-8"))
                    return
                else:
                    self.send_response(200)
                    self.end_headers_with_cors()
                    self.wfile.write(json.dumps({"success": False, "cancelled": True}).encode("utf-8"))
                    return
            except Exception as e:
                self.send_response(500)
                self.end_headers_with_cors()
                self.wfile.write(json.dumps({"success": False, "error": str(e)}).encode("utf-8"))
            return

        if parsed.path == "/api/convert":
            try:
                data = json.loads(post_data.decode("utf-8"))
                mode = data.get("mode", "centerline")
                corners = data.get("corners", "sharp")
                threshold = int(data.get("threshold", 210))
                autocrop = data.get("autocrop", True)
                add_frame = data.get("addFrame", False)
                
                if "base64" in data and data["base64"]:
                    b64 = data["base64"].split(",")[-1]
                    img_bytes = base64.b64decode(b64)
                elif "filePath" in data:
                    full_p = Path(data["filePath"]) if Path(data["filePath"]).is_absolute() else (BASE_DIR / data["filePath"])
                    with open(full_p, "rb") as f:
                        img_bytes = f.read()
                else:
                    raise ValueError("Görsel verisi sağlanmadı")
                    
                svg_result = process_image_to_svg(
                    img_bytes,
                    mode=mode,
                    corners=corners,
                    threshold=threshold,
                    autocrop=autocrop,
                    add_frame=add_frame
                )
                
                save_to = data.get("saveTo")
                saved_path = None
                if save_to:
                    dest = Path(save_to) if Path(save_to).is_absolute() else (BASE_DIR / save_to)
                    with open(dest, "w", encoding="utf-8") as f:
                        f.write(svg_result)
                    saved_path = str(dest)
                    
                self.send_response(200)
                self.end_headers_with_cors()
                self.wfile.write(json.dumps({
                    "success": True,
                    "svg": svg_result,
                    "savedPath": saved_path
                }).encode("utf-8"))
            except Exception as e:
                self.send_response(500)
                self.end_headers_with_cors()
                self.wfile.write(json.dumps({"success": False, "error": str(e)}).encode("utf-8"))
            return

        if parsed.path == "/api/batch-convert":
            try:
                data = json.loads(post_data.decode("utf-8"))
                folder_name = data.get("folder")
                mode = data.get("mode", "centerline")
                corners = data.get("corners", "sharp")
                threshold = int(data.get("threshold", 210))
                autocrop = data.get("autocrop", True)
                add_frame = data.get("addFrame", False)
                
                target_dir = BASE_DIR / folder_name
                results = []
                
                for ext in ["*.png", "*.jpg", "*.jpeg", "*.pdf"]:
                    for img_file in target_dir.glob(ext):
                        if img_file.name.startswith(".tmp"):
                            continue
                        with open(img_file, "rb") as f:
                            img_b = f.read()
                        svg = process_image_to_svg(
                            img_b,
                            mode=mode,
                            corners=corners,
                            threshold=threshold,
                            autocrop=autocrop,
                            add_frame=add_frame
                        )
                        
                        suffix = "_tek_hat.svg" if mode == "centerline" else "_kontur.svg"
                        out_svg = target_dir / f"{img_file.stem}{suffix}"
                        with open(out_svg, "w", encoding="utf-8") as f:
                            f.write(svg)
                        results.append(out_svg.name)
                        
                self.send_response(200)
                self.end_headers_with_cors()
                self.wfile.write(json.dumps({
                    "success": True,
                    "convertedCount": len(results),
                    "files": results
                }).encode("utf-8"))
            except Exception as e:
                self.send_response(500)
                self.end_headers_with_cors()
                self.wfile.write(json.dumps({"success": False, "error": str(e)}).encode("utf-8"))
            return

        self.send_response(404)
        self.end_headers()

def run():
    server_address = ("127.0.0.1", PORT)
    httpd = HTTPServer(server_address, LaserAppHandler)
    print(f"svgapp çalışıyor: http://127.0.0.1:{PORT}")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nSunucu kapatıldı.")

if __name__ == "__main__":
    run()
