# Quickstart: Self Test App MVP

## Prerequisites

- Rust (latest stable)
- Node.js LTS + npm
- PostgreSQL 14+
- MiniMax API key

## Environment Variables

Create `.env` files for backend and frontend as needed.

### Backend (`backend/.env`)

```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/self_test
JWT_SECRET=change-me
JWT_REFRESH_SECRET=change-me
AI_API_KEY=your_ai_key
AI_BASE_URL=https://api.example.com/v1
AI_MODEL=your-model-id
UPLOAD_DIR=./uploads
```

### Frontend (`frontend/.env`)

```
VITE_API_BASE_URL=http://localhost:3000
```

## Local Setup (Planned)

1. Start PostgreSQL and create a database named `self_test`.
2. Run backend migrations.
3. Start the backend server.
4. Start the frontend dev server.

## Key Endpoints

See API contracts in [specs/002-self-test-mvp/contracts/openapi.yaml](specs/002-self-test-mvp/contracts/openapi.yaml).
