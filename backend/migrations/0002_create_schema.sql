-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Learning Materials table
CREATE TABLE learning_materials (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    file_url TEXT,
    type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Generation Configs table
CREATE TABLE generation_configs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    material_id UUID NOT NULL REFERENCES learning_materials(id),
    mcq_single_count INT NOT NULL DEFAULT 0,
    mcq_multi_count INT NOT NULL DEFAULT 0,
    fill_blank_count INT NOT NULL DEFAULT 0,
    language TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Question Sets table (needed for generation and tests)
CREATE TABLE question_sets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    material_id UUID NOT NULL REFERENCES learning_materials(id),
    config_id UUID REFERENCES generation_configs(id),
    status TEXT NOT NULL DEFAULT 'created',
    raw_ai_response TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- Generation Jobs table
CREATE TABLE generation_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    material_id UUID NOT NULL REFERENCES learning_materials(id),
    status TEXT NOT NULL DEFAULT 'queued',
    question_set_id UUID REFERENCES question_sets(id),
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Questions table
CREATE TABLE questions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_set_id UUID NOT NULL REFERENCES question_sets(id),
    type TEXT NOT NULL,
    prompt TEXT NOT NULL,
    options TEXT[] NOT NULL DEFAULT '{}',
    correct_answer TEXT NOT NULL,
    explanation TEXT,
    order_index INT NOT NULL DEFAULT 0
);

-- Test Attempts table
CREATE TABLE test_attempts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    question_set_id UUID NOT NULL REFERENCES question_sets(id),
    score FLOAT,
    max_score FLOAT,
    status TEXT NOT NULL DEFAULT 'in-progress',
    feedback_summary TEXT,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- Answers table
CREATE TABLE answers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    attempt_id UUID NOT NULL REFERENCES test_attempts(id),
    question_id UUID NOT NULL REFERENCES questions(id),
    user_answer TEXT NOT NULL,
    is_correct BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
