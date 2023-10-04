-- Crée une fonction d'ajout de colonne
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

-- On exécute la fonction d'ajout de colonne sur la table food
SELECT create_column_if_not_exists('food', 'expiration_date', 'DATE', 'CURRENT_DATE');

-- Et si il y a déjà des données (qui ont donc une date d'expiration 'null'),
-- alors on modifie leurs date d'expiration à 'aujourd'hui'.
UPDATE food
SET expiration_date = current_date
WHERE expiration_date IS NULL;

-- Pour la DEMO
INSERT INTO food (id, name, expiration_date)
SELECT id, name, expiration_date
FROM (VALUES
          ('bf3f933e-397b-49aa-8081-7455a2c6f043'::UUID, 'Kiwi', CURRENT_DATE - 5),
          ('2fd6252a-bc98-4f22-92e9-6546e4ca4d67'::UUID, 'Jambon', CURRENT_DATE),
          ('c1f5d9be-149d-49f6-b166-cc04f6b56b74'::UUID, 'Salade', CURRENT_DATE + 7),
          ('e3df9269-dbc6-4113-a70e-8d1dbf54fa51'::UUID, 'Oeuf', CURRENT_DATE + 2),
          ('db26700c-6289-45fb-84fe-7201563a056f'::UUID, 'Fraise', CURRENT_DATE - 3)
     ) AS new_rows(id, name, expiration_date);
