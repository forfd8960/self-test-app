# Data Model: Self Test App MVP

## Entities

### User
- **Fields**: id (UUID), username, password_hash, created_at, updated_at, status
- **Relationships**: has many LearningMaterial, QuestionSet, TestAttempt
- **Validation**: username format, unique username, password min length (8+)
- **State**: status ∈ {active, disabled}

### LearningMaterial
- **Fields**: id (UUID), user_id, original_filename, storage_path, file_type (pdf|docx|txt), file_size_bytes, uploaded_at, extracted_text_status, extracted_text
- **Relationships**: belongs to User; has many QuestionSet
- **Validation**: file_type allowed; file_size ≤ configured limit
- **State**: extracted_text_status ∈ {pending, processing, ready, failed}

### GenerationConfig
- **Fields**: id (UUID), user_id, material_id, mcq_single_count, mcq_multi_count, fill_blank_count, language, created_at
- **Relationships**: belongs to User and LearningMaterial; has one QuestionSet
- **Validation**: counts ≥ 0; total questions > 0

### QuestionSet
- **Fields**: id (UUID), user_id, material_id, config_id, status, created_at, completed_at, raw_ai_response
- **Relationships**: belongs to User, LearningMaterial, GenerationConfig; has many Question
- **State**: status ∈ {queued, generating, ready, failed}

### Question
- **Fields**: id (UUID), question_set_id, type (single|multiple|blank), prompt, options[], correct_answer, explanation, order_index
- **Relationships**: belongs to QuestionSet; has many Answer
- **Validation**: options required for choice types; correct_answer format matches type

### TestAttempt
- **Fields**: id (UUID), user_id, question_set_id, started_at, submitted_at, score_percent, feedback_summary
- **Relationships**: belongs to User and QuestionSet; has many Answer
- **Validation**: score_percent in 0–100

### Answer
- **Fields**: id (UUID), attempt_id, question_id, response, is_correct
- **Relationships**: belongs to TestAttempt and Question
- **Validation**: response format matches question type

## Relationships Overview

- User 1..* LearningMaterial
- User 1..* QuestionSet
- User 1..* TestAttempt
- LearningMaterial 1..* QuestionSet
- GenerationConfig 1..1 QuestionSet
- QuestionSet 1..* Question
- TestAttempt 1..* Answer

## State Transitions

- **LearningMaterial**: pending → processing → ready | failed
- **QuestionSet**: queued → generating → ready | failed
- **TestAttempt**: started (created_at) → submitted (submitted_at set)
