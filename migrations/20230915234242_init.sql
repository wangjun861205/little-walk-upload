-- Add migration script here
CREATE OR REPLACE FUNCTION set_uploaded_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.uploaded_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS uploaded_file (
    id SERIAL NOT NULL,
    filename VARCHAR(255) NOT NULL,
    content_type VARCHAR(255) NOT NULL,
    fetch_token VARCHAR(255) NOT NULL,
    uploader_id  INT NOT NULL,
    uploaded_at TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY (id)
);

CREATE TRIGGER fill_uploaded_file_uploaded_at 
BEFORE INSERT ON uploaded_file
FOR EACH ROW EXECUTE PROCEDURE set_uploaded_at();