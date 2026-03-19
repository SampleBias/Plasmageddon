import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// -- Repos --
export interface Repo {
  id: string;
  name: string;
  description: string;
  created_at: string;
  updated_at: string;
}

export const repos = {
  create: (name: string, description: string) =>
    invoke<Repo>("create_repo", { name, description }),
  get: (id: string) => invoke<Repo>("get_repo", { id }),
  list: () => invoke<Repo[]>("list_repos"),
  update: (id: string, name: string, description: string) =>
    invoke<Repo>("update_repo", { id, name, description }),
  delete: (id: string) => invoke<void>("delete_repo", { id }),
  search: (query: string) => invoke<Repo[]>("search_repos", { query }),
};

// -- Constructs --
export interface Construct {
  id: string;
  repo_id: string;
  name: string;
  description: string;
  topology: string;
  tags: string[];
  sequence: string;
  created_at: string;
  updated_at: string;
}

export interface ConstructPart {
  id: string;
  construct_id: string;
  part_id: string;
  position: number;
  strand: number;
  order_index: number;
}

export interface ConstructVersion {
  id: string;
  construct_id: string;
  version_num: number;
  sequence: string;
  parts_json: string;
  snapshot_at: string;
}

export const constructs = {
  create: (repoId: string, name: string, description: string, topology: string) =>
    invoke<Construct>("create_construct", {
      repoId,
      name,
      description,
      topology,
    }),
  get: (id: string) => invoke<Construct>("get_construct", { id }),
  list: (repoId: string) => invoke<Construct[]>("list_constructs", { repoId }),
  update: (id: string, name: string, description: string, topology: string, tags: string[], sequence: string) =>
    invoke<Construct>("update_construct", {
      id,
      name,
      description,
      topology,
      tags,
      sequence,
    }),
  delete: (id: string) => invoke<void>("delete_construct", { id }),
  search: (query: string) => invoke<Construct[]>("search_constructs", { query }),
  listVersions: (constructId: string) =>
    invoke<ConstructVersion[]>("list_versions", { constructId }),
  revert: (constructId: string, versionId: string) =>
    invoke<Construct>("revert_construct", { constructId, versionId }),
  addPart: (constructId: string, partId: string, position: number, strand: number, orderIndex: number) =>
    invoke<ConstructPart>("add_construct_part", {
      constructId,
      partId,
      position,
      strand,
      orderIndex,
    }),
  getParts: (constructId: string) =>
    invoke<ConstructPart[]>("get_construct_parts", { constructId }),
  removePart: (id: string) => invoke<void>("remove_construct_part", { id }),
  reorderParts: (constructId: string, partIds: string[]) =>
    invoke<void>("reorder_construct_parts", { constructId, partIds }),
  flipPart: (id: string, strand: number) =>
    invoke<void>("flip_construct_part", { id, strand }),
};

// -- Parts Library --
export interface Part {
  id: string;
  name: string;
  part_type: string;
  sequence: string;
  description: string;
  metadata: Record<string, unknown>;
  created_at: string;
}

export const parts = {
  create: (name: string, partType: string, sequence: string, description: string) =>
    invoke<Part>("create_part", { name, partType, sequence, description }),
  get: (id: string) => invoke<Part>("get_part", { id }),
  list: (partType?: string) => invoke<Part[]>("list_parts", { partType: partType ?? null }),
  search: (query: string) => invoke<Part[]>("search_parts", { query }),
  delete: (id: string) => invoke<void>("delete_part", { id }),
  seed: () => invoke<void>("seed_parts"),
};

// -- Sequence Tools --
export interface CutSite {
  enzyme: string;
  position: number;
  is_unique: boolean;
}

export interface GCResult {
  overall: number;
  window_data: number[];
  window_size: number;
}

export interface TmResult {
  tm_basic: number;
  tm_nearest_neighbor: number;
  length: number;
}

export interface ORF {
  start: number;
  stop: number;
  frame: number;
  length_aa: number;
  strand: string;
}

export const sequence = {
  restrictionSites: (seq: string) =>
    invoke<CutSite[]>("find_restriction_sites", { sequence: seq }),
  gcContent: (seq: string, windowSize: number) =>
    invoke<GCResult>("compute_gc_content", { sequence: seq, windowSize }),
  meltingTemp: (seq: string) => invoke<TmResult>("compute_melting_temp", { sequence: seq }),
  findOrfs: (seq: string, minAa: number) =>
    invoke<ORF[]>("find_orfs", { sequence: seq, minAa }),
  find: (seq: string, query: string, useRegex: boolean) =>
    invoke<[number, number][]>("find_in_sequence", { sequence: seq, query, useRegex }),
  translate: (dna: string) => invoke<string>("translate_sequence", { dna }),
};

// -- AI --
export interface CompilerInput {
  aa_sequences: { name: string; sequence: string; chain_type: string }[];
  architecture: string;
  host: string;
}

export interface CompilerOutput {
  constructs: {
    name: string;
    dna_sequence: string;
    parts: { name: string; part_type: string; sequence: string; description: string }[];
    signal_peptide: string;
  }[];
  notes: string;
}

export interface SimulatorInput {
  construct_name: string;
  sequence: string;
  parts_summary: string[];
  host: string;
  copy_number: number;
  time_hours: number;
}

export interface SimulatorOutput {
  mrna_level: number;
  protein_level: number;
  bottlenecks: string[];
  developability_score: number;
  time_course: { time_h: number; mrna: number; protein: number }[];
  notes: string;
}

export interface PartSuggestion {
  part_name: string;
  part_type: string;
  reason: string;
  score: number;
}

export interface ChatMessage {
  id: string;
  construct_id: string | null;
  role: string;
  content: string;
  created_at: string;
}

export interface BiosecurityResult {
  is_flagged: boolean;
  hits: { organism: string; description: string; risk_level: string; region_start: number; region_end: number }[];
  screening_provider: string;
}

export const ai = {
  compile: (input: CompilerInput) => invoke<CompilerOutput>("run_compiler", { input }),
  simulate: (input: SimulatorInput) => invoke<SimulatorOutput>("run_simulator", { input }),
  chat: (constructId: string | null, message: string) =>
    invoke<string>("ai_chat", { constructId, message }),
  chatStream: (constructId: string | null, message: string) =>
    invoke<string>("ai_chat_stream", { constructId, message }),
  getChatHistory: (constructId: string | null) =>
    invoke<ChatMessage[]>("get_chat_history", { constructId }),
  clearChat: (constructId: string | null) =>
    invoke<void>("clear_chat_history", { constructId }),
  suggestParts: (currentParts: string[]) =>
    invoke<PartSuggestion[]>("suggest_parts", { currentParts }),
};

export const io = {
  importFile: (path: string, repoId: string) =>
    invoke<{ construct_id: string; name: string; sequence_length: number; parts_found: number }>(
      "import_file",
      { path, repoId },
    ),
  exportGenbank: (constructId: string, path: string) =>
    invoke<void>("export_genbank", { constructId, path }),
  exportFasta: (constructId: string, path: string) =>
    invoke<void>("export_fasta", { constructId, path }),
  exportCsv: (constructId: string, path: string) =>
    invoke<void>("export_csv", { constructId, path }),
  screenSequence: (seq: string) => invoke<BiosecurityResult>("screen_sequence", { sequence: seq }),
};

export const settings = {
  get: (key: string) => invoke<string | null>("get_setting", { key }),
  set: (key: string, value: string) => invoke<void>("set_setting", { key, value }),
};

// -- Notebooks --
export interface Notebook {
  id: string;
  repo_id: string;
  title: string;
  content: string;
  created_at: string;
  updated_at: string;
}

export const notebooks = {
  create: (repoId: string, title: string) =>
    invoke<Notebook>("create_notebook", { repoId, title }),
  get: (id: string) => invoke<Notebook>("get_notebook", { id }),
  list: (repoId: string) => invoke<Notebook[]>("list_notebooks", { repoId }),
  update: (id: string, title: string, content: string) =>
    invoke<Notebook>("update_notebook", { id, title, content }),
  delete: (id: string) => invoke<void>("delete_notebook", { id }),
};

// -- ODE Simulator --
export interface CircuitPart {
  name: string;
  part_type: string;
}

export interface OdeParams {
  alpha?: number;
  alpha0?: number;
  beta?: number;
  n?: number;
  k_m?: number;
}

export interface SpeciesTimeCourse {
  name: string;
  color: string;
  values: number[];
}

export interface OdeSimResult {
  time_points: number[];
  species: SpeciesTimeCourse[];
  steady_state: boolean;
  period_hours: number | null;
  circuit_type: string;
  notes: string;
}

export const odeSimulator = {
  run: (
    circuitType: string,
    parts: CircuitPart[],
    durationHours: number,
    dt: number,
    parameters?: OdeParams,
  ) =>
    invoke<OdeSimResult>("run_ode_simulation", {
      circuitType,
      parts,
      durationHours,
      dt,
      parameters: parameters ?? null,
    }),
  detectCircuit: (parts: CircuitPart[]) =>
    invoke<string>("detect_circuit", { parts }),
  seedBacterialDemo: () =>
    invoke<string>("seed_bacterial_demo"),
};

export const events = {
  onAiChunk: (handler: (chunk: string) => void) =>
    listen<string>("ai:chunk", (e) => handler(e.payload)),
  onAiDone: (handler: () => void) =>
    listen<boolean>("ai:done", () => handler()),
  onAiResponse: (handler: (response: string) => void) =>
    listen<string>("ai:response", (e) => handler(e.payload)),
};
