import { api } from "../../lib/api";

export interface TestAttempt {
  id: string;
  user_id: string;
  question_set_id: string;
  started_at: string;
  submitted_at: string | null;
  score_percent: number | null;
  feedback_summary: string | null;
}

export interface AttemptAnswerDetail {
  question_id: string;
  user_response: string;
  is_correct: boolean;
  correct_answer: string;
  explanation: string | null;
  prompt: string;
}

export interface TestAttemptDetail {
  attempt: TestAttempt;
  answers: AttemptAnswerDetail[];
}

export const getHistory = async (token: string) => {
  return api.get<TestAttempt[]>("/tests", token);
};

export const getHistoryDetail = async (token: string, id: string) => {
  return api.get<TestAttemptDetail>(`/tests/${id}`, token);
};
