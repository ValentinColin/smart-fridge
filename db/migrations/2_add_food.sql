-- Ajoute des valeurs initial dans la base de donnée pour l'exemple
INSERT INTO food (name)
SELECT name
FROM (VALUES ('Banane'), ('Banane'), ('Yaourt'), ('Viande'), ('Tomate')) AS new_rows(name)
WHERE NOT EXISTS (SELECT 1 FROM food);
