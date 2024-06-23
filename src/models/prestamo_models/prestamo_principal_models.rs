use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoModelo {
    pub id_prestamo: i32,
    pub prestamo_migrado: i32,
    pub prestamo_proyecto: i32,
    pub prestamo_linea_credito: i32,
    pub prestamo_fira: i32,
    pub prestamo_seguro: i32,
    pub prestamo_fecha: i32,
    pub prestamo_autorizacion: i32,
    pub prestamo_entrega: i32,
    pub prestamo_garantias: i32,
    pub prestamo_documentos: i32,
    pub prestamo_crediticio: i32,
    pub prestamo_lavado: i32,
    pub prestamo_analisis_capacidad: i32,
    pub prestamo_anterior: i32,
    pub prestamo_ultimo_calculo: i32,
    pub prestamo_estructura_datos: i32,
    pub prestamo_legal: i32,
    pub prestamo_adicional: i32,
    pub prestamo_impresiones: i32,
}
