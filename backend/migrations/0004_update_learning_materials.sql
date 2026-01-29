-- Update learning_materials to match code refactor
ALTER TABLE learning_materials RENAME COLUMN title TO original_filename;
ALTER TABLE learning_materials RENAME COLUMN file_url TO storage_path;
ALTER TABLE learning_materials RENAME COLUMN type TO file_type;
ALTER TABLE learning_materials RENAME COLUMN status TO extracted_text_status;
ALTER TABLE learning_materials RENAME COLUMN created_at TO uploaded_at;
ALTER TABLE learning_materials RENAME COLUMN content TO extracted_text;

ALTER TABLE learning_materials ALTER COLUMN extracted_text DROP NOT NULL;
ALTER TABLE learning_materials ADD COLUMN file_size_bytes BIGINT NOT NULL DEFAULT 0;

ALTER TABLE learning_materials DROP COLUMN updated_at;
