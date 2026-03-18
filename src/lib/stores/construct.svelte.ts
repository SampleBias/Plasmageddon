export interface Construct {
  id: string;
  repo_id: string;
  name: string;
  description: string;
  topology: "circular" | "linear";
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
  strand: 1 | -1;
  order_index: number;
}

export interface ConstructPartWithInfo extends ConstructPart {
  name: string;
  part_type: string;
  sequence: string;
  description: string;
}

export interface SelectionState {
  type: "part" | "range" | "none";
  partId?: string;
  start?: number;
  end?: number;
}

export const currentConstruct = $state<{ value: Construct | null }>({ value: null });
export const constructParts = $state<{ value: ConstructPartWithInfo[] }>({ value: [] });
export const selection = $state<{ value: SelectionState }>({ value: { type: "none" } });
export const isDirty = $state({ value: false });

export function selectPart(partId: string) {
  const part = constructParts.value.find((p) => p.id === partId);
  if (part) {
    selection.value = {
      type: "part",
      partId,
      start: part.position,
      end: part.position + part.sequence.length,
    };
  }
}

export function selectRange(start: number, end: number) {
  selection.value = { type: "range", start, end };
}

export function clearSelection() {
  selection.value = { type: "none" };
}
