-- Add down migration script here
ALTER TABLE calles ALTER COLUMN id_calle DROP DEFAULT;
DROP SEQUENCE IF EXISTS id_calle_seq;

