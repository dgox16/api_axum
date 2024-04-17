CREATE TABLE domicilios (
    id_domicilio SERIAL PRIMARY KEY,
    cp VARCHAR(5),
    colonia VARCHAR(50),
    calle_id BIGINT,
    entre_calle_id BIGINT,
    y_calle_id BIGINT,
    numero_exterior VARCHAR(10),
    numero_interior VARCHAR(10),
    geolocalizacion VARCHAR(50),
    FOREIGN KEY (calle_id) REFERENCES calles(id_calle) ON DELETE RESTRICT,
    FOREIGN KEY (entre_calle_id) REFERENCES calles(id_calle) ON DELETE RESTRICT,
    FOREIGN KEY (y_calle_id) REFERENCES calles(id_calle) ON DELETE RESTRICT
);