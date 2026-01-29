type Question = {
  id: string;
  prompt: string;
};

type Props = {
  questions: Question[];
};

export function QuestionList({ questions }: Props) {
  if (!questions.length) {
    return <p>No questions generated yet.</p>;
  }

  return (
    <ol>
      {questions.map((question) => (
        <li key={question.id}>{question.prompt}</li>
      ))}
    </ol>
  );
}
