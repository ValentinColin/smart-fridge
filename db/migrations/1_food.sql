-- On crée la table si elle n'existe pas
CREATE TABLE IF NOT EXISTS food (
    id                  bigserial PRIMARY KEY,
    name                text,
    expiration_date     DATE DEFAULT current_date
);

-- et si elle existe déjà on crée une nouvelle colonne
CREATE OR REPLACE FUNCTION create_column_if_not_exists(
    _table_name text,
    _column_name text,
    _column_type text,
    _column_default_value text
)
    RETURNS void AS $$
BEGIN
    IF NOT EXISTS (
        SELECT column_name
        FROM information_schema.columns
        WHERE table_name = _table_name
          AND column_name = _column_name
    ) THEN
        EXECUTE 'ALTER TABLE ' || _table_name || ' ADD COLUMN ' || _column_name || ' ' || _column_type || ' DEFAULT ' || _column_default_value;
    END IF;
END;
$$ LANGUAGE plpgsql;
SELECT create_column_if_not_exists('food', 'expiration_date', 'DATE', 'CURRENT_DATE');
