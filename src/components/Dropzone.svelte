<script lang="ts">
  import { pickFile } from "../lib/api";

  export let fileName: string = "";
  export let onFileSelected: (data: {
    fileName: string;
    filePath?: string;
    dataUri?: string;
    isPdf: boolean;
  }) => void;

  let isDragOver = false;

  async function handleClick() {
    try {
      const res = await pickFile();
      if (res.success && res.data_uri) {
        onFileSelected({
          fileName: res.file_name || "cizim",
          filePath: res.file_path,
          dataUri: res.data_uri,
          isPdf: res.is_pdf,
        });
      }
    } catch (err) {
      console.error("Dosya seçimi açılamadı, yerel dosya inputuna yönlendiriliyor:", err);
      // Yerel input fallback
      fileInputElement?.click();
    }
  }

  let fileInputElement: HTMLInputElement;

  function handleFileInputChange(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      processFile(input.files[0]);
    }
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragOver = false;
    if (e.dataTransfer && e.dataTransfer.files && e.dataTransfer.files[0]) {
      processFile(e.dataTransfer.files[0]);
    }
  }

  function processFile(file: File) {
    const isPdf = file.name.toLowerCase().endsWith(".pdf");
    const reader = new FileReader();
    reader.onload = () => {
      const dataUri = reader.result as string;
      onFileSelected({
        fileName: file.name,
        dataUri,
        isPdf,
      });
    };
    reader.readAsDataURL(file);
  }
</script>

<div class="dropzone-wrapper">
  <div class="section-title">Görsel / PDF Çizim Seç</div>
  <div
    class="dropzone"
    class:dragover={isDragOver}
    on:click={handleClick}
    on:keydown={(e) => e.key === 'Enter' && handleClick()}
    on:dragover|preventDefault={() => (isDragOver = true)}
    on:dragleave={() => (isDragOver = false)}
    on:drop={handleDrop}
    role="button"
    tabindex="0"
  >
    <svg viewBox="0 0 24 24" class="upload-icon">
      <path
        d="M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM14 13v4h-4v-4H7l5-5 5 5h-3z"
      />
    </svg>
    <div class="file-name-label">
      {fileName || "Görsel veya PDF Seçmek İçin Tıklayın"}
    </div>
    <div class="sub-label">PNG, JPG, PDF Sürükleyip Bırakabilirsiniz</div>
  </div>
  <input
    type="file"
    bind:this={fileInputElement}
    accept="image/*,.pdf,application/pdf"
    style="display:none"
    on:change={handleFileInputChange}
  />
</div>

<style>
  .dropzone-wrapper {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .section-title {
    font-size: 0.82rem;
    text-transform: uppercase;
    color: var(--text-muted);
    font-weight: 700;
    letter-spacing: 0.5px;
  }
  .dropzone {
    border: 2px dashed #475569;
    border-radius: 8px;
    padding: 20px 14px;
    text-align: center;
    cursor: pointer;
    transition: all 0.2s ease;
    background: rgba(255, 255, 255, 0.02);
    user-select: none;
  }
  .dropzone:hover,
  .dropzone.dragover {
    border-color: var(--accent);
    background: rgba(56, 189, 248, 0.06);
    transform: translateY(-1px);
  }
  .upload-icon {
    width: 36px;
    height: 36px;
    fill: var(--text-muted);
    margin-bottom: 8px;
    transition: fill 0.2s;
  }
  .dropzone:hover .upload-icon {
    fill: var(--accent);
  }
  .file-name-label {
    font-weight: 600;
    font-size: 0.9rem;
    color: var(--text);
    word-break: break-all;
  }
  .sub-label {
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-top: 4px;
  }
</style>
