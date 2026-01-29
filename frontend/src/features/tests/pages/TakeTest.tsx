import { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useAuthStore } from '../../auth/store';
import { getQuestions, Question } from '../../generation/api';
import { submitTest, TestResultResponse } from '../api';
import { ScoreSummary } from '../components/ScoreSummary';

export const TakeTest = () => {
  const { setId } = useParams();
  const token = useAuthStore(s => s.token);
  const [questions, setQuestions] = useState<Question[]>([]);
  const [answers, setAnswers] = useState<Record<string, string>>({});
  const [result, setResult] = useState<TestResultResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const navigate = useNavigate();

  useEffect(() => {
    if (setId && token) {
      getQuestions(setId, token)
        .then(setQuestions)
        .catch(console.error)
        .finally(() => setLoading(false));
    }
  }, [setId, token]);

  const handleAnswerChange = (questionId: string, value: string) => {
    setAnswers(prev => ({ ...prev, [questionId]: value }));
  };

  const handleMultiSelectChange = (questionId: string, option: string, checked: boolean) => {
    const current = answers[questionId] ? answers[questionId].split(',') : [];
    let next: string[];
    if (checked) {
      next = [...current, option];
    } else {
      next = current.filter(o => o !== option);
    }
    setAnswers(prev => ({ ...prev, [questionId]: next.join(',') }));
  };

  const handleSubmit = async () => {
    if (!setId || !token) return;
    const submission = {
        question_set_id: setId,
        answers: Object.entries(answers).map(([qid, resp]) => ({
            question_id: qid,
            response: resp
        }))
    };
    try {
        const res = await submitTest(token, submission);
        setResult(res);
    } catch (e) {
        alert("Failed to submit test");
        console.error(e);
    }
  };

  if (result) return <ScoreSummary result={result} onHome={() => navigate('/')} />;
  if (loading) return <div className="p-8 text-center">Loading questions...</div>;
  if (!questions.length) return <div className="p-8 text-center">No questions found.</div>;

  return (
    <div className="p-4 max-w-3xl mx-auto space-y-8">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold text-gray-800">Take Test</h1>
        <button onClick={() => navigate('/')} className="text-gray-600 hover:underline">Cancel</button>
      </div>

      {questions.map((q, idx) => (
        <div key={q.id} className="p-6 border rounded-lg shadow-sm bg-white">
          <div className="flex items-start gap-4 mb-4">
            <span className="bg-blue-100 text-blue-800 font-bold px-3 py-1 rounded-full text-sm">
              Q{idx + 1}
            </span>
            <p className="font-semibold text-lg text-gray-800 mt-1">{q.prompt}</p>
          </div>

          <div className="ml-14">
            {q.question_type === 'single' && (
              <div className="space-y-3">
                {q.options.map((opt) => (
                  <label key={opt} className="flex items-center gap-3 cursor-pointer p-2 hover:bg-gray-50 rounded">
                    <input
                      type="radio"
                      name={q.id}
                      value={opt}
                      checked={answers[q.id] === opt}
                      onChange={(e) => handleAnswerChange(q.id, e.target.value)}
                      className="w-5 h-5 text-blue-600"
                    />
                    <span className="text-gray-700">{opt}</span>
                  </label>
                ))}
              </div>
            )}

            {q.question_type === 'multiple' && (
              <div className="space-y-3">
                {q.options.map((opt) => (
                  <label key={opt} className="flex items-center gap-3 cursor-pointer p-2 hover:bg-gray-50 rounded">
                    <input
                      type="checkbox"
                      value={opt}
                      checked={answers[q.id]?.split(',').includes(opt)}
                      onChange={(e) => handleMultiSelectChange(q.id, opt, e.target.checked)}
                      className="w-5 h-5 text-blue-600 rounded"
                    />
                    <span className="text-gray-700">{opt}</span>
                  </label>
                ))}
                <p className="text-sm text-gray-500 mt-2 italic">* Select all that apply</p>
              </div>
            )}

            {q.question_type === 'blank' && (
              <input
                type="text"
                placeholder="Type your answer here..."
                value={answers[q.id] || ''}
                onChange={(e) => handleAnswerChange(q.id, e.target.value)}
                className="w-full p-3 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500 focus:outline-none"
              />
            )}
          </div>
        </div>
      ))}

      <div className="flex justify-end pt-6 pb-12">
        <button 
          onClick={handleSubmit} 
          className="px-8 py-3 bg-blue-600 text-white font-bold rounded-lg hover:bg-blue-700 shadow transition-transform transform active:scale-95"
        >
          Submit Test
        </button>
      </div>
    </div>
  );
}
