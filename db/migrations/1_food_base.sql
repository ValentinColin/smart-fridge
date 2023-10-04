-- On ajoute l'extension uuid
--CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- On crée la table
CREATE TABLE food (
    id                  uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name                text
);

-- Pour la DEMO
INSERT INTO food (id, name)
SELECT id, name
FROM (VALUES
          ('9b96b52a-7cdc-4437-9317-27e5a5c59d0e'::UUID, 'Banane'),
          ('e62d8a71-643d-4f2f-bc63-8cf6a3e57d8e'::UUID, 'Banane'),
          ('a557b4d3-dbe8-44bb-8179-3496495b49f5'::UUID, 'Yaourt'),
          ('a5110003-3ab1-48e9-a4c4-f4a7a2fdbc3a'::UUID, 'Viande'),
          ('ea8eeb89-d98b-4a13-88dd-1a3eccc1f8f2'::UUID, 'Tomate')
     ) AS new_rows(id, name);
