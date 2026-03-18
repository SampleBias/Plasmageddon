# Plasmageddon

> End of bad designs

Native desktop CAD for synthetic biology / genetic design. Built with **Tauri 2.0** (Rust backend) + **Svelte 5** (frontend) + **PixiJS 8** (2D visualization).

## Prerequisites

### Linux (Arch)

```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl gtk3 libappindicator-gtk3 librsvg
```

### macOS

Xcode Command Line Tools (comes with Xcode).

### Both platforms

- [Rust](https://rustup.rs/) 1.75+
- [Node.js](https://nodejs.org/) 20+
- Tauri CLI: `cargo install tauri-cli --version "^2"`

## Setup

```bash
npm install
cargo tauri dev
```

## Architecture

```
src/                  # Svelte 5 frontend (TypeScript)
├── lib/
│   ├── api/          # Typed Tauri invoke wrappers
│   ├── components/   # UI components (layout, editor, parts, tools, AI)
│   ├── pixi/         # PixiJS canvas renderers (schematic, sequence, circular)
│   └── stores/       # Svelte 5 reactive state ($state runes)
└── routes/           # Page components (Home, Repos, Editor, Settings)

src-tauri/            # Rust backend
├── src/
│   ├── ai/           # GLM-5 + Groq API clients, compiler, simulator, chat
│   ├── bio/          # Restriction sites, GC%, Tm, ORF, codon tables
│   ├── biosecurity/  # SecureDNA screening
│   ├── commands/     # All #[tauri::command] functions
│   ├── db/           # SQLite schema, repos, constructs, parts, chat
│   └── parsers/      # GenBank, FASTA, SnapGene (.dna) parsers
```

## Features

- **Three synchronized views**: Schematic (drag-drop parts), Sequence (annotated nucleotides), Circular Map
- **Parts library**: 30+ built-in genetic parts (promoters, terminators, origins, markers, tags, linkers, signal peptides)
- **Sequence tools**: Restriction sites (30 common enzymes), GC content, melting temperature, ORF finder, sequence search
- **AI Compiler** (GLM-5): Paste AA sequences → get codon-optimized DNA constructs with signal peptides and backbone
- **AI Simulator** (GLM-5): Predict expression levels, bottlenecks, and developability scores
- **AI Chat** (GLM-5): Context-aware assistant with streaming responses and per-construct conversation history
- **Part Match** (Groq): Suggest complementary parts from your library
- **Import/Export**: GenBank, FASTA, SnapGene .dna import; GenBank/FASTA/CSV export
- **Biosecurity**: SecureDNA screening on export
- **Auto-versioning**: Full construct history with revert
- **Local-first**: SQLite database, API keys stored locally

## Configuration

Set your API keys in **Settings** (the gear icon):

- **GLM-5 / Zhipu AI**: Used for Compiler, Simulator, and AI Chat
- **Groq Cloud**: Used for Part Match and lightweight tasks

## License

Proprietary
