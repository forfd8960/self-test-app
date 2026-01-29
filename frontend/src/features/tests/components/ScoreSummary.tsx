import React from 'react';
import { TestResultResponse } from '../api';

interface ScoreSummaryProps {
  result: TestResultResponse;
  onHome: () => void;
}

export const ScoreSummary: React.FC<ScoreSummaryProps> = ({ result, onHome }) => {
  return (
    <div className="space-y-6 max-w-2xl mx-auto">
      <div className="bg-white p-6 rounded-lg shadow-md text-center border border-gray-200">
        <h2 className="text-2xl font-bold text-gray-800">Test Completed!</h2>
        <div className={`text-5xl font-extrabold my-4 ${result.score_percent >= 70 ? 'text-green-600' : 'text-orange-500'}`}>
          {result.score_percent.toFixed(0)}%
        </div>
        <div className="bg-blue-50 p-4 rounded text-left border border-blue-100">
          <h4 className="font-semibold text-blue-800 mb-2">AI Feedback:</h4>
          <p className="text-gray-700 whitespace-pre-wrap">{result.feedback}</p>
        </div>
      </div>

      <div className="space-y-4">
        <h3 className="text-xl font-semibold text-gray-800">Answer Key</h3>
        {result.correct_answers.map((ans, idx) => (
          <div 
            key={ans.question_id} 
            className={`p-4 rounded-lg border ${ans.is_correct ? 'border-green-200 bg-green-50' : 'border-red-200 bg-red-50'}`}
          >
            <div className="flex justify-between items-start mb-2">
              <span className="font-bold text-gray-700">Question {idx + 1}</span>
              <span className={`px-2 py-1 rounded text-xs font-bold ${ans.is_correct ? 'bg-green-200 text-green-800' : 'bg-red-200 text-red-800'}`}>
                {ans.is_correct ? 'CORRECT' : 'INCORRECT'}
              </span>
            </div>
            
            {!ans.is_correct && (
              <div className="mt-2 text-sm">
                <p className="font-semibold text-red-700">Correct Answer:</p>
                <p className="text-gray-800">{ans.correct_answer}</p>
              </div>
            )}
            
            {ans.explanation && (
              <div className="mt-2 text-sm text-gray-600 border-t border-gray-200 pt-2">
                <span className="font-semibold">Explanation:</span> {ans.explanation}
              </div>
            )}
          </div>
        ))}
      </div>

      <div className="flex justify-center pt-4">
        <button 
          onClick={onHome}
          className="px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 font-medium transition-colors"
        >
          Back to Dashboard
        </button>
      </div>
    </div>
  );
};
