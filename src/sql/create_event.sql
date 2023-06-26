INSERT INTO events(name)
VALUES ($1)
RETURNING $table_fields;
