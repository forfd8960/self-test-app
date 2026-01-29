const DEFAULT_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? "http://localhost:3000";

type RequestOptions = {
  token?: string;
  headers?: Record<string, string>;
};

async function request<T>(path: string, options: RequestInit = {}, extra: RequestOptions = {}): Promise<T> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    ...extra.headers,
  };

  if (extra.token) {
    headers.Authorization = `Bearer ${extra.token}`;
  }

  const response = await fetch(`${DEFAULT_BASE_URL}${path}`, {
    ...options,
    headers,
  });

  if (!response.ok) {
    const message = await response.text();
    throw new Error(message || "Request failed");
  }

  return response.json() as Promise<T>;
}

export const api = {
  get: <T>(path: string, token?: string, headers?: Record<string, string>) =>
    request<T>(path, { method: "GET" }, { token, headers }),
  post: <T, B = unknown>(path: string, body: B, token?: string, headers?: Record<string, string>) =>
    request<T>(path, { method: "POST", body: JSON.stringify(body) }, { token, headers }),
};
