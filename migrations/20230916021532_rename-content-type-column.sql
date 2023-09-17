-- Add migration script here
ALTER TABLE uploaded_files RENAME COLUMN content_type TO mime_type;
