import { Application, Container, Graphics, Text, TextStyle, FederatedPointerEvent } from "pixi.js";
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

const PART_HEIGHT = 50;
const PART_GAP = 12;
const PART_MIN_WIDTH = 80;
const LABEL_STYLE = new TextStyle({
  fontFamily: "Inter, system-ui, sans-serif",
  fontSize: 11,
  fill: 0xffffff,
  wordWrap: true,
  wordWrapWidth: 120,
});

export class SchematicApp {
  app: Application;
  container: Container;
  private partsContainer: Container;
  private parts: ConstructPartWithInfo[] = [];
  private onPartClick: (partId: string) => void = () => {};
  private onPartDrop: (partId: string, index: number) => void = () => {};
  private onReorder: (partIds: string[]) => void = () => {};
  private dragging: { sprite: Container; startX: number; partId: string; origIndex: number } | null = null;

  constructor() {
    this.app = new Application();
    this.container = new Container();
    this.partsContainer = new Container();
    this.container.addChild(this.partsContainer);
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
    this.container.y = 40;
    this.container.x = 20;

    this.app.stage.eventMode = "static";
    this.app.stage.on("pointermove", this.onDragMove.bind(this));
    this.app.stage.on("pointerup", this.onDragEnd.bind(this));
  }

  setCallbacks(opts: {
    onPartClick: (partId: string) => void;
    onPartDrop: (partId: string, index: number) => void;
    onReorder: (partIds: string[]) => void;
  }) {
    this.onPartClick = opts.onPartClick;
    this.onPartDrop = opts.onPartDrop;
    this.onReorder = opts.onReorder;
  }

  render(parts: ConstructPartWithInfo[], selectedPartId?: string) {
    this.parts = parts;
    this.partsContainer.removeChildren();

    let xOffset = 0;

    for (let i = 0; i < parts.length; i++) {
      const part = parts[i];
      const color = PART_COLORS[part.part_type] ?? 0x9ca3af;
      const width = Math.max(PART_MIN_WIDTH, Math.min(part.sequence.length / 10, 200));
      const isSelected = part.id === selectedPartId;

      const partContainer = new Container();
      partContainer.x = xOffset;
      partContainer.y = 60;
      partContainer.eventMode = "static";
      partContainer.cursor = "pointer";

      const gfx = new Graphics();

      if (part.part_type === "promoter") {
        this.drawPromoterGlyph(gfx, width, PART_HEIGHT, color, part.strand);
      } else if (part.part_type === "terminator") {
        this.drawTerminatorGlyph(gfx, width, PART_HEIGHT, color);
      } else if (part.part_type === "cds") {
        this.drawCDSGlyph(gfx, width, PART_HEIGHT, color, part.strand);
      } else {
        this.drawGenericGlyph(gfx, width, PART_HEIGHT, color);
      }

      if (isSelected) {
        gfx.setStrokeStyle({ width: 2, color: 0xffffff });
        gfx.rect(-2, -2, width + 4, PART_HEIGHT + 4);
        gfx.stroke();
      }

      partContainer.addChild(gfx);

      const label = new Text({ text: part.name, style: LABEL_STYLE });
      label.x = 6;
      label.y = PART_HEIGHT + 6;
      label.anchor.set(0, 0);
      partContainer.addChild(label);

      const typeLabel = new Text({
        text: part.part_type,
        style: new TextStyle({ fontSize: 9, fill: 0x9498b8, fontFamily: "Inter, system-ui, sans-serif" }),
      });
      typeLabel.x = 6;
      typeLabel.y = 6;
      partContainer.addChild(typeLabel);

      if (part.strand === -1) {
        const flipLabel = new Text({
          text: "\u2190",
          style: new TextStyle({ fontSize: 14, fill: 0xffffff, fontFamily: "Inter, system-ui, sans-serif" }),
        });
        flipLabel.x = width - 18;
        flipLabel.y = PART_HEIGHT / 2 - 8;
        partContainer.addChild(flipLabel);
      }

      const partId = part.id;
      partContainer.on("pointerdown", (e: FederatedPointerEvent) => {
        this.onPartClick(partId);
        this.dragging = {
          sprite: partContainer,
          startX: e.global.x,
          partId,
          origIndex: i,
        };
      });

      // Connector arrow between parts
      if (i < parts.length - 1) {
        const arrow = new Graphics();
        arrow.setStrokeStyle({ width: 2, color: 0x3a3d52 });
        arrow.moveTo(width + 2, PART_HEIGHT / 2);
        arrow.lineTo(width + PART_GAP - 2, PART_HEIGHT / 2);
        arrow.stroke();
        arrow.setStrokeStyle({ width: 2, color: 0x3a3d52 });
        arrow.moveTo(width + PART_GAP - 6, PART_HEIGHT / 2 - 4);
        arrow.lineTo(width + PART_GAP - 2, PART_HEIGHT / 2);
        arrow.lineTo(width + PART_GAP - 6, PART_HEIGHT / 2 + 4);
        arrow.stroke();
        partContainer.addChild(arrow);
      }

      this.partsContainer.addChild(partContainer);
      xOffset += width + PART_GAP;
    }

    // Backbone line
    if (parts.length > 0) {
      const backbone = new Graphics();
      backbone.setStrokeStyle({ width: 3, color: 0x282a3a });
      backbone.moveTo(0, 60 + PART_HEIGHT / 2);
      backbone.lineTo(xOffset - PART_GAP, 60 + PART_HEIGHT / 2);
      backbone.stroke();
      this.partsContainer.addChildAt(backbone, 0);
    }
  }

  private drawPromoterGlyph(g: Graphics, w: number, h: number, color: number, strand: number) {
    g.rect(0, 0, w, h).fill({ color, alpha: 0.2 });

    if (strand === 1) {
      g.poly([0, h, 0, h * 0.3, w * 0.7, h * 0.3, w, h * 0.15, w * 0.7, 0, w * 0.7, h * 0.3, w, h * 0.3, w, h]);
    } else {
      g.poly([w, h, w, h * 0.3, w * 0.3, h * 0.3, 0, h * 0.15, w * 0.3, 0, w * 0.3, h * 0.3, 0, h * 0.3, 0, h]);
    }
    g.fill({ color, alpha: 0.6 });
  }

  private drawTerminatorGlyph(g: Graphics, w: number, h: number, color: number) {
    g.rect(0, 0, w, h).fill({ color, alpha: 0.3 });
    g.setStrokeStyle({ width: 3, color });
    g.moveTo(w / 2, h);
    g.lineTo(w / 2, h * 0.2);
    g.stroke();
    g.setStrokeStyle({ width: 3, color });
    g.moveTo(w * 0.2, h * 0.2);
    g.lineTo(w * 0.8, h * 0.2);
    g.stroke();
  }

  private drawCDSGlyph(g: Graphics, w: number, h: number, color: number, strand: number) {
    if (strand === 1) {
      g.poly([0, 0, w - 15, 0, w, h / 2, w - 15, h, 0, h]);
    } else {
      g.poly([15, 0, w, 0, w, h, 15, h, 0, h / 2]);
    }
    g.fill({ color, alpha: 0.6 });
  }

  private drawGenericGlyph(g: Graphics, w: number, h: number, color: number) {
    g.roundRect(0, 0, w, h, 4).fill({ color, alpha: 0.4 });
    g.roundRect(0, 0, w, h, 4).stroke({ width: 1, color, alpha: 0.8 });
  }

  private onDragMove(e: FederatedPointerEvent) {
    if (!this.dragging) return;
    const dx = e.global.x - this.dragging.startX;
    this.dragging.sprite.alpha = 0.6;
    this.dragging.sprite.x += dx;
    this.dragging.startX = e.global.x;
  }

  private onDragEnd() {
    if (!this.dragging) return;
    this.dragging.sprite.alpha = 1;

    const dropX = this.dragging.sprite.x;
    let newIndex = 0;
    let accX = 0;

    for (let i = 0; i < this.parts.length; i++) {
      const w = Math.max(PART_MIN_WIDTH, Math.min(this.parts[i].sequence.length / 10, 200));
      if (dropX > accX + w / 2) {
        newIndex = i + 1;
      }
      accX += w + PART_GAP;
    }

    if (newIndex !== this.dragging.origIndex) {
      const ids = this.parts.map((p) => p.id);
      const [moved] = ids.splice(this.dragging.origIndex, 1);
      ids.splice(newIndex > this.dragging.origIndex ? newIndex - 1 : newIndex, 0, moved);
      this.onReorder(ids);
    }

    this.dragging = null;
    this.render(this.parts);
  }

  handleExternalDrop(partId: string, x: number) {
    let index = 0;
    let accX = 0;

    for (let i = 0; i < this.parts.length; i++) {
      const w = Math.max(PART_MIN_WIDTH, Math.min(this.parts[i].sequence.length / 10, 200));
      if (x > accX + w / 2) index = i + 1;
      accX += w + PART_GAP;
    }

    this.onPartDrop(partId, index);
  }

  resize() {
    this.app.resize();
  }

  destroy() {
    this.app.destroy(true);
  }
}
