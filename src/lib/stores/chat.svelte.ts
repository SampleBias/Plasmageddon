export interface ChatMessage {
  id: string;
  construct_id: string | null;
  role: "system" | "user" | "assistant";
  content: string;
  created_at: string;
}

export const chatMessages = $state<{ value: ChatMessage[] }>({ value: [] });
export const chatLoading = $state({ value: false });
export const chatInput = $state({ value: "" });
