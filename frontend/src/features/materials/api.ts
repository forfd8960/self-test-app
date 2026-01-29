import { api } from "../../lib/api";

export type Material = {
  id: string;
  original_filename: string;
  file_type: string;
  file_size_bytes: number;
  uploaded_at: string;
  extracted_text_status: string;
};

export async function listMaterials(token: string | null): Promise<Material[]> {
  return api.get<Material[]>("/materials", token ?? undefined);
}
