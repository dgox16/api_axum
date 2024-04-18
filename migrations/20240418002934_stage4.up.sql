-- Add up migration script here
ALTER TABLE calles
ALTER COLUMN nombre SET NOT NULL;

-- Agregar restricción NOT NULL a la columna tipo
ALTER TABLE calles
ALTER COLUMN tipo SET NOT NULL;