import { Application, Container, Graphics, Text, TextStyle } from "pixi.js";
import type { ConstructPartWithInfo } from "../../stores/construct.svelte";

const BASE_COLORS: Record<string, number> = {
  A: 0x4ade80,
  T: 0xf87171,
  G: 0xfacc15,
  C: 0x60a5fa,
};

const CHAR_WIDTH = 10;
const CHAR_HEIGHT = 16;
const LINE_HEIGHT = 22;
const BASES_PER_ROW = 80;
const GUTTER_WIDTH = 70;
const ANNOTATION_HEIGHT = 8;

const BASE_STYLE = new TextStyle({
  fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
  fontSize: 12,
  fill: 0xffffff,
});

const GUTTER_STYLE = new TextStyle({
  fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
  fontSize: 10,
  fill: 0x6b7094,
});

const PART_COLORS: Record<string, number> = {
  promoter: 0xf59e0b,
  cds: 0x3b82f6,
  terminator: 0xef4444,
  ori: 0x8b5cf6,
  marker: 0x10b981,
  tag: 0xec4899,
  linker: 0x6b7280,
  signal_peptide: 0xf97316,
  regulatory: 0x06b6d4,
  other: 0x9ca3af,
};

export class SequenceApp {
  app: Application;
  private container: Container;
  private scrollY = 0;
  private totalHeight = 0;
  private sequence = "";
  private parts: ConstructPartWithInfo[] = [];
  private selectedRange: { start: number; end: number } | null = null;
  private onSelect: (start: number, end: number) => void = () => {};

  constructor() {
    this.app = new Application();
    this.container = new Container();
  }

  async init(canvas: HTMLCanvasElement) {
    await this.app.init({
      canvas,
      resizeTo: canvas.parentElement!,
      background: 0x0f1117,
      antialias: true,
      autoDensity: true,
      resolution: window.devicePixelRatio || 1,
    });
    this.app.stage.addChild(this.container);

    canvas.addEventListener("wheel", (e) => {
      e.preventDefault();
      this.scrollY = Math.max(0, Math.min(this.totalHeight - this.app.screen.height, this.scrollY + e.deltaY));
      this.renderVisible();
    });
  }

  setCallbacks(opts: { onSelect: (start: number, end: number) => void }) {
    this.onSelect = opts.onSelect;
  }

  render(sequence: string, parts: ConstructPartWithInfo[], selectedRange?: { start: number; end: number }) {
    this.sequence = sequence.toUpperCase();
    this.parts = parts;
    this.selectedRange = selectedRange ?? null;
    const rows = Math.ceil(this.sequence.length / BASES_PER_ROW);
    const annotationRows = parts.length > 0 ? 2 : 0;
    this.totalHeight = rows * (LINE_HEIGHT + ANNOTATION_HEIGHT * annotationRows) + 100;
    this.renderVisible();
  }

  private renderVisible() {
    this.container.removeChildren();

    if (!this.sequence) return;

    const screenH = this.app.screen.height;
    const annotRows = this.parts.length > 0 ? 2 : 0;
    const rowTotalH = LINE_HEIGHT + ANNOTATION_HEIGHT * annotRows;

    const startRow = Math.max(0, Math.floor(this.scrollY / rowTotalH) - 1);
    const endRow = Math.min(
      Math.ceil(this.sequence.length / BASES_PER_ROW),
      Math.ceil((this.scrollY + screenH) / rowTotalH) + 1,
    );

    for (let row = startRow; row < endRow; row++) {
      const y = row * rowTotalH - this.scrollY;
      const baseStart = row * BASES_PER_ROW;
      const baseEnd = Math.min(baseStart + BASES_PER_ROW, this.sequence.length);
      const rowSeq = this.sequence.slice(baseStart, baseEnd);

      // Line number
      const gutterText = new Text({
        text: String(baseStart + 1),
        style: GUTTER_STYLE,
      });
      gutterText.x = 4;
      gutterText.y = y + annotRows * ANNOTATION_HEIGHT;
      this.container.addChild(gutterText);

      // Annotation tracks for this row
      for (const part of this.parts) {
        const partStart = part.position;
        const partEnd = part.position + part.sequence.length;

        if (partEnd <= baseStart || partStart >= baseEnd) continue;

        const drawStart = Math.max(partStart, baseStart) - baseStart;
        const drawEnd = Math.min(partEnd, baseEnd) - baseStart;
        const color = PART_COLORS[part.part_type] ?? 0x9ca3af;

        const annot = new Graphics();
        annot.rect(
          GUTTER_WIDTH + drawStart * CHAR_WIDTH,
          y,
          (drawEnd - drawStart) * CHAR_WIDTH,
          ANNOTATION_HEIGHT - 1,
        ).fill({ color, alpha: 0.5 });
        this.container.addChild(annot);
      }

      // Selection highlight
      if (this.selectedRange) {
        const selStart = Math.max(this.selectedRange.start, baseStart) - baseStart;
        const selEnd = Math.min(this.selectedRange.end, baseEnd) - baseStart;
        if (selEnd > selStart) {
          const sel = new Graphics();
          sel.rect(
            GUTTER_WIDTH + selStart * CHAR_WIDTH,
            y + annotRows * ANNOTATION_HEIGHT,
            (selEnd - selStart) * CHAR_WIDTH,
            LINE_HEIGHT,
          ).fill({ color: 0x818cf8, alpha: 0.2 });
          this.container.addChild(sel);
        }
      }

      // Base characters with 10-bp spacing
      for (let i = 0; i < rowSeq.length; i++) {
        const base = rowSeq[i];
        const color = BASE_COLORS[base] ?? 0x9ca3af;
        const spacing = i > 0 && i % 10 === 0 ? 3 : 0;

        const charText = new Text({
          text: base,
          style: new TextStyle({
            fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
            fontSize: 12,
            fill: color,
          }),
        });
        charText.x = GUTTER_WIDTH + i * CHAR_WIDTH + spacing;
        charText.y = y + annotRows * ANNOTATION_HEIGHT;
        this.container.addChild(charText);
      }
    }
  }

  highlight(start: number, end: number) {
    this.selectedRange = { start, end };
    const rowTotalH = LINE_HEIGHT + (this.parts.length > 0 ? 2 : 0) * ANNOTATION_HEIGHT;
    const targetRow = Math.floor(start / BASES_PER_ROW);
    this.scrollY = Math.max(0, targetRow * rowTotalH - this.app.screen.height / 3);
    this.renderVisible();
  }

  resize() {
    this.app.resize();
    this.renderVisible();
  }

  destroy() {
    this.app.destroy(true);
  }
}
