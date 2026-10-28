export class PanZoomViewer {
  container: HTMLElement;
  content: HTMLElement;
  badge?: HTMLElement | null;
  onUpdate?: (scale: number, panX: number, panY: number) => void;

  scale: number = 1.0;
  panX: number = 0;
  panY: number = 0;
  isDragging: boolean = false;
  startX: number = 0;
  startY: number = 0;

  private onMouseDownBound: (e: MouseEvent) => void;
  private onMouseMoveBound: (e: MouseEvent) => void;
  private onMouseUpBound: (e: MouseEvent) => void;
  private onWheelBound: (e: WheelEvent) => void;
  private onDblClickBound: (e: MouseEvent) => void;

  constructor(
    container: HTMLElement,
    content: HTMLElement,
    badge?: HTMLElement | null,
    onUpdate?: (scale: number, panX: number, panY: number) => void
  ) {
    this.container = container;
    this.content = content;
    this.badge = badge;
    this.onUpdate = onUpdate;

    this.onMouseDownBound = this.onMouseDown.bind(this);
    this.onMouseMoveBound = this.onMouseMove.bind(this);
    this.onMouseUpBound = this.onMouseUp.bind(this);
    this.onWheelBound = this.onWheel.bind(this);
    this.onDblClickBound = this.onDblClick.bind(this);

    this.initEvents();
  }

  private initEvents() {
    this.container.addEventListener('mousedown', this.onMouseDownBound);
    window.addEventListener('mousemove', this.onMouseMoveBound);
    window.addEventListener('mouseup', this.onMouseUpBound);
    this.container.addEventListener('wheel', this.onWheelBound, { passive: false });
    this.container.addEventListener('dblclick', this.onDblClickBound);
  }

  private onMouseDown(e: MouseEvent) {
    if (e.button !== 0) return;
    // Native resim sürüklemesini engelle
    e.preventDefault();
    this.isDragging = true;
    this.startX = e.clientX - this.panX;
    this.startY = e.clientY - this.panY;
    this.container.classList.add('dragging');
  }

  private onMouseMove(e: MouseEvent) {
    if (!this.isDragging) return;
    this.panX = e.clientX - this.startX;
    this.panY = e.clientY - this.startY;
    this.applyTransform();
  }

  private onMouseUp(_e: MouseEvent) {
    if (this.isDragging) {
      this.isDragging = false;
      this.container.classList.remove('dragging');
    }
  }

  private onDblClick(e: MouseEvent) {
    e.preventDefault();
    this.fitToView();
  }

  private onWheel(e: WheelEvent) {
    e.preventDefault();
    e.stopPropagation();
    const rect = this.container.getBoundingClientRect();
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;

    // Trackpad ve fare tekerleği hassasiyeti
    const delta = e.deltaY;
    const factor = delta < 0 ? 1.15 : 0.87;
    this.zoomAt(mouseX, mouseY, factor);
  }

  zoomAt(x: number, y: number, factor: number) {
    const oldScale = this.scale;
    let newScale = oldScale * factor;
    newScale = Math.min(Math.max(newScale, 0.05), 40); // %5 ile %4000 arası

    this.panX = x - (x - this.panX) * (newScale / oldScale);
    this.panY = y - (y - this.panY) * (newScale / oldScale);
    this.scale = newScale;

    this.applyTransform();
  }

  zoomStep(direction: number) {
    const rect = this.container.getBoundingClientRect();
    const cx = rect.width / 2;
    const cy = rect.height / 2;
    const factor = direction > 0 ? 1.25 : 0.8;
    this.zoomAt(cx, cy, factor);
  }

  fitToView() {
    const rect = this.container.getBoundingClientRect();
    if (rect.width === 0 || rect.height === 0) return;

    const target = this.content.querySelector('svg, img') as HTMLElement | SVGSVGElement | null;
    if (!target) {
      this.reset();
      return;
    }

    let tw = 0;
    let th = 0;

    if (target instanceof HTMLImageElement) {
      tw = target.naturalWidth || target.clientWidth || target.getBoundingClientRect().width;
      th = target.naturalHeight || target.clientHeight || target.getBoundingClientRect().height;
    } else if (target instanceof SVGSVGElement) {
      if (target.viewBox && target.viewBox.baseVal && target.viewBox.baseVal.width > 0) {
        tw = target.viewBox.baseVal.width;
        th = target.viewBox.baseVal.height;
      } else if (target.width && target.width.baseVal && target.width.baseVal.value > 0) {
        tw = target.width.baseVal.value;
        th = target.height.baseVal.value;
      } else {
        const b = target.getBoundingClientRect();
        tw = b.width;
        th = b.height;
      }
    } else {
      const b = target.getBoundingClientRect();
      tw = b.width;
      th = b.height;
    }

    if (tw > 0 && th > 0) {
      const pad = 30;
      const availW = Math.max(rect.width - pad * 2, 40);
      const availH = Math.max(rect.height - pad * 2, 40);
      this.scale = Math.min(availW / tw, availH / th, 1.0);
      this.panX = (rect.width - tw * this.scale) / 2;
      this.panY = (rect.height - th * this.scale) / 2;
    } else {
      this.scale = 1.0;
      this.panX = 20;
      this.panY = 20;
    }

    this.applyTransform();
  }

  reset() {
    const rect = this.container.getBoundingClientRect();
    this.scale = 1.0;
    const target = this.content.querySelector('svg, img') as HTMLElement | SVGSVGElement | null;
    if (target && rect.width > 0) {
      const tw = target.getBoundingClientRect().width || 200;
      const th = target.getBoundingClientRect().height || 200;
      this.panX = (rect.width - tw) / 2;
      this.panY = (rect.height - th) / 2;
    } else {
      this.panX = 0;
      this.panY = 0;
    }
    this.applyTransform();
  }

  applyTransform() {
    if (this.content) {
      this.content.style.transform = `translate3d(${this.panX}px, ${this.panY}px, 0px) scale(${this.scale})`;
    }
    if (this.badge) {
      this.badge.innerText = `${Math.round(this.scale * 100)}%`;
    }
    if (this.onUpdate) {
      this.onUpdate(this.scale, this.panX, this.panY);
    }
  }

  destroy() {
    this.container.removeEventListener('mousedown', this.onMouseDownBound);
    window.removeEventListener('mousemove', this.onMouseMoveBound);
    window.removeEventListener('mouseup', this.onMouseUpBound);
    this.container.removeEventListener('wheel', this.onWheelBound);
    this.container.removeEventListener('dblclick', this.onDblClickBound);
  }
}
