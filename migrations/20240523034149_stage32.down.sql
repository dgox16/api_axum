ALTER TABLE personas DROP CONSTRAINT IF EXISTS personas_tipo_fkey;
ALTER TABLE personas DROP CONSTRAINT IF EXISTS personas_usuario_actualizo_fkey;
ALTER TABLE personas DROP CONSTRAINT IF EXISTS personas_barrio_fkey;
ALTER TABLE personas DROP CONSTRAINT IF EXISTS personas_ciudad_fkey;
ALTER TABLE personas DROP CONSTRAINT IF EXISTS personas_calle_fkey;
ALTER TABLE personas DROP CONSTRAINT IF EXISTS personas_pais_nacimiento_fkey;
ALTER TABLE personas DROP CONSTRAINT IF EXISTS personas_persona_conyuge_fkey;

ALTER TABLE personas ADD CONSTRAINT personas_tipo_fkey FOREIGN KEY (tipo) REFERENCES tipos_persona (id_tipo_persona);
ALTER TABLE personas ADD CONSTRAINT personas_usuario_actualizo_fkey FOREIGN KEY (usuario_actualizo) REFERENCES usuarios (id);
ALTER TABLE personas ADD CONSTRAINT personas_barrio_fkey FOREIGN KEY (barrio) REFERENCES barrios (id_barrio);
ALTER TABLE personas ADD CONSTRAINT personas_ciudad_fkey FOREIGN KEY (ciudad) REFERENCES ciudades (id_ciudad);
ALTER TABLE personas ADD CONSTRAINT personas_calle_fkey FOREIGN KEY (calle) REFERENCES calles (id_calle);
ALTER TABLE personas ADD CONSTRAINT personas_pais_nacimiento_fkey FOREIGN KEY (pais_nacimiento) REFERENCES paises (id_pais);
ALTER TABLE personas ADD CONSTRAINT personas_persona_conyuge_fkey FOREIGN KEY (persona_conyuge) REFERENCES personas (id_persona);
