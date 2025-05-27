-- Create function to generate slug only if NEW.slug is null or empty
CREATE OR REPLACE FUNCTION generate_slug()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.slug IS NULL OR LENGTH(TRIM(NEW.slug)) = 0 THEN
        NEW.slug := LOWER(REPLACE(TRIM(NEW.name), ' ', '-'));
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DO $$
DECLARE
    tbl TEXT;
BEGIN
    FOREACH tbl IN ARRAY ARRAY['account_types', 'permissions', 'roles']
    LOOP
        EXECUTE format('
            CREATE TRIGGER set_%I_slug
            BEFORE INSERT OR UPDATE OF name
            ON %I
            FOR EACH ROW
            EXECUTE FUNCTION generate_slug();
        ', tbl, tbl, tbl, tbl);
    END LOOP;
END $$;

