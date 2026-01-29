import { api } from "../../lib/api";

export type GenerationRequest = {
  material_id: string;
  mcq_single_count: number;
  mcq_multi_count: number;
  fill_blank_count: number;
  language?: string;
};

export type GenerationJob = {
  id: string;
  status: string;
  question_set_id?: string | null;
  error_message?: string | null;
};

export type Question = {
  id: string;
  question_set_id: string;
  question_type: "single" | "multiple" | "blank";
  prompt: string;
  options: string[];
  order_index: number;
};

export async function getQuestions(setId: string, token: string | null): Promise<Question[]> {
  return api.get<Question[]>(`/generation/sets/${setId}/questions`, token ?? undefined);
}

export async function createGenerationJob(
  payload: GenerationRequest,
  token: string | null,
): Promise<GenerationJob> {
  return api.post<GenerationJob, GenerationRequest>("/generation", payload, token ?? undefined);
}
