-- This file should undo anything in `up.sql`
DELETE FROM counter
WHERE
  value = 1;

DROP TABLE counter;