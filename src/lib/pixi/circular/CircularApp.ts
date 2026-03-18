import { Application, Container, Graphics, Text, TextStyle } from "pixi.js";
import type { ConstructPartWithInfo } from "../../stores/construct.svelte";

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

export class CircularApp {
  app: Application;
  private container: Container;
  private onPartClick: (partId: string) => void = () => {};

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
  }

  setCallbacks(opts: { onPartClick: (partId: string) => void }) {
    this.onPartClick = opts.onPartClick;
  }

  render(
    sequence: string,
    parts: ConstructPartWithInfo[],
    topology: "circular" | "linear",
    selectedPartId?: string,
  ) {
    this.container.removeChildren();

    const w = this.app.screen.width;
    const h = this.app.screen.height;
    const cx = w / 2;
    const cy = h / 2;
    const seqLen = sequence.length;

    if (topology === "linear") {
      this.renderLinear(cx, cy, w, h, seqLen, parts, selectedPartId);
      return;
    }

    const radius = Math.min(w, h) * 0.35;
    const ringWidth = 20;

    // Outer backbone ring
    const backbone = new Graphics();
    backbone.setStrokeStyle({ width: 3, color: 0x282a3a });
    backbone.circle(cx, cy, radius);
    backbone.stroke();
    this.container.addChild(backbone);

    // Part arcs
    for (const part of parts) {
      if (seqLen === 0) continue;
      const startAngle = (part.position / seqLen) * Math.PI * 2 - Math.PI / 2;
      const endAngle = ((part.position + part.sequence.length) / seqLen) * Math.PI * 2 - Math.PI / 2;
      const color = PART_COLORS[part.part_type] ?? 0x9ca3af;
      const isSelected = part.id === selectedPartId;

      const arc = new Graphics();
      arc.setStrokeStyle({ width: isSelected ? ringWidth + 4 : ringWidth, color, alpha: isSelected ? 0.9 : 0.6 });
      arc.arc(cx, cy, radius, startAngle, endAngle);
      arc.stroke();

      arc.eventMode = "static";
      arc.cursor = "pointer";
      const pid = part.id;
      arc.on("pointerdown", () => this.onPartClick(pid));

      this.container.addChild(arc);

      // Label along the arc
      const midAngle = (startAngle + endAngle) / 2;
      const labelRadius = radius + ringWidth / 2 + 18;
      const lx = cx + Math.cos(midAngle) * labelRadius;
      const ly = cy + Math.sin(midAngle) * labelRadius;

      const label = new Text({
        text: part.name,
        style: new TextStyle({
          fontFamily: "Inter, system-ui, sans-serif",
          fontSize: 10,
          fill: isSelected ? 0xffffff : 0xc4c7df,
        }),
      });
      label.anchor.set(0.5, 0.5);
      label.x = lx;
      label.y = ly;

      // Rotate label to follow the arc
      let rotation = midAngle + Math.PI / 2;
      if (rotation > Math.PI / 2 && rotation < (3 * Math.PI) / 2) {
        rotation += Math.PI;
      }
      label.rotation = rotation;

      this.container.addChild(label);
    }

    // Center text (bp count)
    const centerText = new Text({
      text: `${seqLen.toLocaleString()}\nbp`,
      style: new TextStyle({
        fontFamily: "Inter, system-ui, sans-serif",
        fontSize: 20,
        fontWeight: "bold",
        fill: 0xe8e9f2,
        align: "center",
      }),
    });
    centerText.anchor.set(0.5, 0.5);
    centerText.x = cx;
    centerText.y = cy;
    this.container.addChild(centerText);

    // Tick marks every 1kb
    if (seqLen > 0) {
      const tickInterval = seqLen < 5000 ? 500 : seqLen < 20000 ? 1000 : 5000;
      for (let bp = 0; bp < seqLen; bp += tickInterval) {
        const angle = (bp / seqLen) * Math.PI * 2 - Math.PI / 2;
        const innerR = radius - 8;
        const outerR = radius + 8;

        const tick = new Graphics();
        tick.setStrokeStyle({ width: 1, color: 0x3a3d52 });
        tick.moveTo(cx + Math.cos(angle) * innerR, cy + Math.sin(angle) * innerR);
        tick.lineTo(cx + Math.cos(angle) * outerR, cy + Math.sin(angle) * outerR);
        tick.stroke();
        this.container.addChild(tick);

        if (bp % (tickInterval * 2) === 0) {
          const tickLabel = new Text({
            text: bp >= 1000 ? `${(bp / 1000).toFixed(bp % 1000 === 0 ? 0 : 1)}k` : String(bp),
            style: new TextStyle({ fontSize: 8, fill: 0x6b7094, fontFamily: "Inter, system-ui, sans-serif" }),
          });
          const labelR = radius - 20;
          tickLabel.anchor.set(0.5, 0.5);
          tickLabel.x = cx + Math.cos(angle) * labelR;
          tickLabel.y = cy + Math.sin(angle) * labelR;
          this.container.addChild(tickLabel);
        }
      }
    }
  }

  private renderLinear(cx: number, cy: number, w: number, h: number, seqLen: number, parts: ConstructPartWithInfo[], selectedPartId?: string) {
    const barWidth = w * 0.8;
    const barHeight = 20;
    const barX = (w - barWidth) / 2;
    const barY = cy - barHeight / 2;

    const backbone = new Graphics();
    backbone.roundRect(barX, barY, barWidth, barHeight, 4).fill({ color: 0x282a3a });
    this.container.addChild(backbone);

    for (const part of parts) {
      if (seqLen === 0) continue;
      const x = barX + (part.position / seqLen) * barWidth;
      const partW = Math.max(4, (part.sequence.length / seqLen) * barWidth);
      const color = PART_COLORS[part.part_type] ?? 0x9ca3af;
      const isSelected = part.id === selectedPartId;

      const rect = new Graphics();
      rect.roundRect(x, barY - 2, partW, barHeight + 4, 2).fill({ color, alpha: isSelected ? 0.9 : 0.6 });
      rect.eventMode = "static";
      rect.cursor = "pointer";
      const pid = part.id;
      rect.on("pointerdown", () => this.onPartClick(pid));
      this.container.addChild(rect);

      const label = new Text({
        text: part.name,
        style: new TextStyle({ fontSize: 9, fill: 0xc4c7df, fontFamily: "Inter, system-ui, sans-serif" }),
      });
      label.x = x + 2;
      label.y = barY - 16;
      this.container.addChild(label);
    }

    const bpLabel = new Text({
      text: `${seqLen.toLocaleString()} bp (linear)`,
      style: new TextStyle({ fontSize: 14, fill: 0xe8e9f2, fontFamily: "Inter, system-ui, sans-serif" }),
    });
    bpLabel.anchor.set(0.5, 0);
    bpLabel.x = cx;
    bpLabel.y = barY + barHeight + 20;
    this.container.addChild(bpLabel);
  }

  resize() {
    this.app.resize();
  }

  destroy() {
    this.app.destroy(true);
  }
}
