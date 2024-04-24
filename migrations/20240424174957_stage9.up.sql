CREATE TABLE polizas_egreso (
    id_poliza_egreso SERIAL PRIMARY KEY,
    poliza INTEGER NOT NULL,
    beneficiario VARCHAR(80) NOT NULL,
    banco INTEGER NOT NULL,
    cheque VARCHAR(80) NOT NULL,
    FOREIGN KEY (banco) REFERENCES bancos(id_banco) ON DELETE RESTRICT,
    FOREIGN KEY (poliza) REFERENCES polizas(id_poliza) ON DELETE RESTRICT
)