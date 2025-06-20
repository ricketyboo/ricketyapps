CREATE OR REPLACE FUNCTION update_modified_column()
    RETURNS TRIGGER
AS
$$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;