use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PrestamoFiraModelo {
    id_prestamo_fira: i32,
    actividad: Option<i32>,
    subrama: Option<i32>,
    cadena: Option<i32>,
    elegible_fira: Option<bool>,
    concepto_fira: Option<i32>,
    grupo_finalidad_prestamo: Option<i32>,
    reciprocidad_subclasificacion: Option<i32>,
    finalidad: Option<i32>,
    descripcion_destino: Option<String>,
    obtendra_recursos_para_pagar: Option<String>,
    fuente_alternativa_de_pago: Option<String>,
    programa: Option<i32>,
    tipo_disposicion: Option<i32>,
    tipo_credito: Option<i32>,
}
