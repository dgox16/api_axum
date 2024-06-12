use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "tipo_servicio", rename_all = "snake_case")]
pub enum TipoServicio {
    Captacion,
    Prestamo,
    Clasificacion,
    Auxiliares,
    InteresesPrestamo,
    InteresesMoratorios,
    InteresesCaptacion,
    Comisiones,
    Servicios,
    Orden,
    Gastos,
    Activos,
    BancosPasiva,
    CarteraPasiva,
    InteresCarteraPasiva,
    Impuestos,
    AcreedoresDeudores,
    TrasladoValores,
    Nomina,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "grupo_servicio", rename_all = "snake_case")]
pub enum GrupoServicio {
    Ahorro,
    Inversion,
    Prestamos,
    Otros,
    Caja,
    Bancos,
    SalvoBuenCobro,
    Boveda,
    TrasladoValores,
    DocumentoEnFirme,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct ServicioModelo {
    pub id_servicio: i32,
    pub clave: i32,
    pub concepto: String,
    pub tipo: TipoServicio,
    pub nombre: String,
    pub nombre_corto: Option<String>,
    pub nombre_regulatorio: Option<String>,
    pub orden: i32,
    pub grupo: GrupoServicio,
    pub sucursal: Option<i32>,
    pub genera_cheque: bool,
    pub genera_iva: bool,
    pub imprime_saldo: bool,
    pub imprime_ficha: bool,
    pub cobro_caja: bool,
    pub seleccionable: bool,
    pub permite_cargos: bool,
    pub permite_abonos: bool,
    pub auditable: bool,
    pub varias_cuentas: bool,
    pub concepto_defecto: Option<String>,
    pub formato_imprimir: Option<String>,
    pub formato_imprimir_2: Option<String>,
    pub servicio_cargo_cargo: i32,
    pub servicio_cargo_abono: i32,
    pub servicio_abono_cargo: i32,
    pub servicio_abono_abono: i32,
    pub deducible: i32,
    pub es_de_orden: bool,
    pub para_cartera_pasiva: bool,
    pub servicio_por_defecto: bool,
    pub es_impuesto: bool,
}
