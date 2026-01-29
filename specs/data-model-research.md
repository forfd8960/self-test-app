# Research: Typical Data Model for Self-Test Apps

**Date**: 2026-01-29

## Scope
Focus on core entities and relationships for: users, materials, question sets, questions, attempts, answers, and feedback.

## Typical Core Entities

### 1) User
- Purpose: Account owner; owns materials, question sets, and attempts.
- Typical fields: id, username, password_hash, created_at, status, locale/timezone.

### 2) LearningMaterial
- Purpose: Uploaded source content for question generation.
- Typical fields: id, user_id, filename, content_type, size_bytes, storage_path, checksum, extracted_text, created_at.

### 3) QuestionSet
- Purpose: Generated test based on a material and configuration.
- Typical fields: id, user_id, material_id, config_json, generation_status, created_at.

### 4) Question
- Purpose: Individual question in a set.
- Typical fields: id, question_set_id, type (mcq_single, mcq_multi, fill_blank), prompt, choices_json, correct_answer_json, explanation, order_index.

### 5) Attempt
- Purpose: A user’s test session for a question set.
- Typical fields: id, user_id, question_set_id, started_at, completed_at, score, feedback_summary, status.

### 6) Answer
- Purpose: User response per question per attempt.
- Typical fields: id, attempt_id, question_id, response_json, is_correct, score, answered_at.

### 7) Feedback
- Purpose: Structured feedback tied to an attempt.
- Typical fields: id, attempt_id, strengths_json, weaknesses_json, recommendations_json, created_at.

## Relationship Map
- User 1—N LearningMaterial
- User 1—N QuestionSet
- LearningMaterial 1—N QuestionSet
- QuestionSet 1—N Question
- User 1—N Attempt
- QuestionSet 1—N Attempt
- Attempt 1—N Answer
- Attempt 1—1 Feedback (or 1—N if feedback is versioned)

## Key Decisions, Rationale, Alternatives

### Decision 1: Separate QuestionSet from Attempt
- Rationale: A single generated test can be retaken; attempts preserve history and allow comparison.
- Alternatives:
  - Store questions directly on Attempt. Simpler but duplicates data per attempt and complicates reuse.

### Decision 2: Store extracted_text on LearningMaterial
- Rationale: Enables deterministic regeneration and fast feedback generation without re-parsing files.
- Alternatives:
  - Store only file path and parse on demand. Lower storage but slower and less reliable for repeat operations.

### Decision 3: Store question config as JSON on QuestionSet
- Rationale: Configs vary (counts, difficulty, topics); JSON avoids migrations for new parameters.
- Alternatives:
  - Normalize config into columns. Better for analytics but increases schema churn.

### Decision 4: Use response_json and correct_answer_json
- Rationale: Supports MCQ single/multi and fill-in-the-blank without multiple tables.
- Alternatives:
  - Separate tables per question type. Stronger type constraints but more joins and duplicated logic.

### Decision 5: Feedback as a separate entity
- Rationale: Feedback can be structured and versioned independently of attempt scoring logic.
- Alternatives:
  - Store feedback_summary on Attempt only. Easier read but loses structure and future expandability.

### Decision 6: Keep question order_index
- Rationale: Stable ordering for display and review, independent of primary key order.
- Alternatives:
  - Rely on created_at or id ordering. Simpler but less explicit and harder to reorder.

### Decision 7: Generation status on QuestionSet
- Rationale: Supports async generation and retries with clear state.
- Alternatives:
  - Use a separate GenerationJob table. More powerful but unnecessary for MVP.

### Decision 8: Answer-level scoring
- Rationale: Enables partial credit and fine-grained analytics.
- Alternatives:
  - Only compute attempt-level score. Simpler but limits feedback quality.

## Common Extensions (Optional)
- Tagging or topic mapping: QuestionTopic, MaterialTopic
- Versioning: QuestionSetVersion or FeedbackVersion
- Sharing: PublicQuestionSet with access controls
- Analytics: Per-question difficulty, time_spent, attempt_count

## Notes for MVP Alignment
- This model supports MVP requirements: upload, generation, answering, scoring, feedback, and history.
- Entities can be implemented as PostgreSQL tables with JSONB fields for flexible payloads.
