import { QuestionList } from "../components/QuestionList";

export function QuestionListPage() {
  return (
    <section>
      <h1>Generated Questions</h1>
      <QuestionList questions={[]} />
    </section>
  );
}
