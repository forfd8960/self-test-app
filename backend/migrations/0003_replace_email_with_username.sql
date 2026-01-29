-- Remove email column
ALTER TABLE users DROP COLUMN email;

-- Add username column
ALTER TABLE users ADD COLUMN username VARCHAR(20) UNIQUE NOT NULL;
