-- Ajoute des valeurs initial dans la base de donnée pour l'exemple
INSERT INTO food (name, expiration_date)
SELECT name, expiration_date
FROM (VALUES
          ('Banane', CURRENT_DATE - 1),
          ('Banane', CURRENT_DATE + 4),
          ('Yaourt', CURRENT_DATE + 3),
          ('Viande', CURRENT_DATE + 2),
          ('Tomate', CURRENT_DATE + 5)
      ) AS new_rows(name, expiration_date)
WHERE NOT EXISTS (SELECT 1 FROM food);

-- Et si il y a déjà des données qui ont une date d'expiration null,
-- alors on set leurs date d'expiration à aujourd'hui.
UPDATE food
SET expiration_date = current_date
WHERE expiration_date IS NULL;