use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoLavadoModelo {
    pub id_prestamo_lavado: i32,
    pub forma_liquidacion: Option<i32>,
    pub antecedente_crediticio: Option<i32>,
    pub tipo_de_cliente: Option<i32>,
    pub observacion_credito: Option<i32>,
}
