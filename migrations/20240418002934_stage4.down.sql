-- Add down migration script here
ALTER TABLE calles
ALTER COLUMN nombre DROP NOT NULL;

-- Eliminar restricción NOT NULL de la columna tipo
ALTER TABLE calles
ALTER COLUMN tipo DROP NOT NULL;