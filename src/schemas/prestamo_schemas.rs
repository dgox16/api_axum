use chrono::NaiveDate;
use serde::Deserialize;

use crate::models::prestamo_models::{
    prestamo_fecha_models::{MinistracionPrestamo, TipoPagaresPrestamo},
    prestamo_seguro_models::TieneSeguroPrestamo,
};

#[derive(Deserialize)]
pub struct CrearPrestamoSchema {
    pub persona: i32,
    pub ministracion: MinistracionPrestamo,
    pub actividad: i32,
    pub monto: f32,
    pub numero_abonos: i32,
    pub periodo_pactado_original: i32,
    pub tipo_de_pagares: TipoPagaresPrestamo,
    pub fuente_financiamiento: i32,
    pub opinion_analista: String,
    pub forma_liquidacion: i32,
    pub fuente_alternativa_de_pago: String,
    pub cuenta: String,
    pub tiene_seguro: TieneSeguroPrestamo,
    pub monto_del_proyecto: f32,
    pub tasa_iva: f32,
    pub compromiso_ahorro: f32,
    pub folio_buro: String,
    pub fecha_buro: NaiveDate,
    pub calificacion_buro: i32,
    pub bc_score: i32,
    pub indice_capacidad_crediticia: i32,
    pub pagos_otras_deudas: f32,
    pub capacidad_pago: f32,
    pub independencia_financiera: f32,
    pub indice_de_liquidez: f32,
    pub interes_otras_deudas: f32,
    pub capacidad_pago_ajustada: f32,
    pub solvencia_general: f32,
    pub rentabilidad: f32,
}
