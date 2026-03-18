export interface Part {
  id: string;
  name: string;
  part_type: string;
  sequence: string;
  description: string;
  metadata: Record<string, unknown>;
  created_at: string;
}

export const partsList = $state<{ value: Part[] }>({ value: [] });
export const partsFilter = $state({ value: "" });
export const partsTypeFilter = $state({ value: "" });
export const partsLoading = $state({ value: false });
