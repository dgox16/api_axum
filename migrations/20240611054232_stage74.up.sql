CREATE TABLE prestamos (
    id_prestamo SERIAL PRIMARY KEY,
    prestamo_migrado INTEGER NOT NULL,
    prestamo_proyecto INTEGER NOT NULL,
    prestamo_linea_credito INTEGER NOT NULL,
    prestamo_fira INTEGER NOT NULL,
    prestamo_seguro INTEGER NOT NULL,
    prestamo_fecha INTEGER NOT NULL,
    prestamo_autorizacion INTEGER NOT NULL,
    prestamo_entrega INTEGER NOT NULL,
    prestamo_garantias INTEGER NOT NULL,
    prestamo_documentos INTEGER NOT NULL,
    prestamo_crediticio INTEGER NOT NULL,
    prestamo_lavado INTEGER NOT NULL,
    prestamo_analisis_capacidad INTEGER NOT NULL,
    prestamo_anterior INTEGER NOT NULL,
    prestamo_ultimo_calculo INTEGER NOT NULL,
    prestamo_estructura_datos INTEGER NOT NULL,
    prestamo_legal INTEGER NOT NULL,
    prestamo_adicional INTEGER NOT NULL,
    prestamo_impresiones INTEGER NOT NULL,
    FOREIGN KEY (prestamo_migrado) REFERENCES prestamos_migrado(id_prestamo_migrado) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_proyecto) REFERENCES prestamos_proyecto(id_prestamo_proyecto) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_linea_credito) REFERENCES prestamos_linea_credito(id_prestamo_linea_credito) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_fira) REFERENCES prestamos_fira(id_prestamo_fira) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_seguro) REFERENCES prestamos_seguro(id_prestamo_seguro) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_fecha) REFERENCES prestamos_fecha(id_prestamo_fecha) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_autorizacion) REFERENCES prestamos_autorizacion(id_prestamo_autorizacion) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_entrega) REFERENCES prestamos_entrega(id_prestamo_entrega) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_garantias) REFERENCES prestamos_garantias(id_prestamo_garantias) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_documentos) REFERENCES prestamos_documentos(id_prestamo_documentos) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_crediticio) REFERENCES prestamos_crediticio(id_prestamo_crediticio) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_lavado) REFERENCES prestamos_lavado(id_prestamo_lavado) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_analisis_capacidad) REFERENCES prestamos_analisis_capacidad(id_prestamo_analisis_capacidad) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_anterior) REFERENCES prestamos_anterior(id_prestamo_anterior) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_ultimo_calculo) REFERENCES prestamos_ultimo_calculo(id_prestamo_ultimo_calculo) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_estructura_datos) REFERENCES prestamos_estructura_datos(id_prestamo_estructura_datos) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_legal) REFERENCES prestamos_legal(id_prestamo_legal) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_adicional) REFERENCES prestamos_adicional(id_prestamo_adicional) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo_impresiones) REFERENCES prestamos_impresiones(id_prestamo_impresiones) ON DELETE RESTRICT
);