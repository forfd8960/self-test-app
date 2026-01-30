import { useState } from "react";

type Question = {
  id: string;
  prompt: string;
};

type Props = {
  questions: Question[];
};

export function QuestionList({ questions }: Props) {
  const [activeIndex, setActiveIndex] = useState(0);

  if (!questions.length) {
    return <p>No questions generated yet.</p>;
  }

  const canGoPrev = activeIndex > 0;
  const canGoNext = activeIndex < questions.length - 1;

  return (
    <div className="w-full">
      <div className="flex items-center justify-between gap-4 mb-4">
        <button
          type="button"
          className="px-3 py-2 rounded-md border border-slate-300 text-slate-700 disabled:opacity-50"
          onClick={() => setActiveIndex((index) => Math.max(0, index - 1))}
          disabled={!canGoPrev}
          aria-label="Previous question"
        >
          ←
        </button>
        <div className="text-sm text-slate-600">
          Question {activeIndex + 1} of {questions.length}
        </div>
        <button
          type="button"
          className="px-3 py-2 rounded-md border border-slate-300 text-slate-700 disabled:opacity-50"
          onClick={() => setActiveIndex((index) => Math.min(questions.length - 1, index + 1))}
          disabled={!canGoNext}
          aria-label="Next question"
        >
          →
        </button>
      </div>
      <div className="overflow-hidden">
        <ol
          className="flex transition-transform duration-300 ease-out"
          style={{ transform: `translateX(-${activeIndex * 100}%)` }}
        >
          {questions.map((question) => (
            <li
              key={question.id}
              className="min-w-full rounded-lg border border-slate-200 bg-white p-6"
            >
              <p className="text-slate-800">{question.prompt}</p>
            </li>
          ))}
        </ol>
      </div>
    </div>
  );
}
