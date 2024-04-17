CREATE SEQUENCE id_calle_seq;
ALTER TABLE calles ALTER COLUMN id_calle SET DEFAULT nextval('id_calle_seq');

