import { api } from "../../lib/api";

export interface SubmitAnswer {
  question_id: string;
  response: string;
}

export interface SubmitTestRequest {
  question_set_id: string;
  answers: SubmitAnswer[];
}

export interface TestResultResponse {
  attempt_id: string;
  score_percent: number;
  feedback: string;
  correct_answers: AnswerResult[];
}

export interface AnswerResult {
  question_id: string;
  is_correct: boolean;
  correct_answer: string;
  explanation: string | null;
}

export const submitTest = async (token: string, data: SubmitTestRequest) => {
  return api.post<TestResultResponse, SubmitTestRequest>("/tests/submit", data, token);
};
