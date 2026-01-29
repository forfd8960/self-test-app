.PHONY: backend frontend dev migrate

# Start the backend server
backend:
	cd backend && cargo run

# Start the frontend server
frontend:
	cd frontend && npm run dev

# Run database migrations
migrate:
	cd backend && cargo sqlx migrate run

# Start both services in parallel
dev:
	make -j2 backend frontend
