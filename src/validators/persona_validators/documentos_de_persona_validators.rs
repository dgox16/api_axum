use axum::{http::StatusCode, Json};

use crate::schemas::persona_schemas::documentos_de_persona_schemas::CrearDocumentoDePersonaSchema;

pub fn validar_nuevo_documento_de_persona(
    documentos_de_persona: &Vec<CrearDocumentoDePersonaSchema>,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    for documento in documentos_de_persona {
        match documento.documento {
            1 => {
                if documento.numero_identificacion.trim().is_empty()
                    || documento.numero_identificacion.trim().len() != 18
                {
                    let respuesta_error = serde_json::json!({
                        "estado": "error",
                        "mensaje": "El formato de la CURP es incorrecta",
                    });
                    return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
                }
            }
            2 => {
                if documento.numero_identificacion.trim().is_empty()
                    || documento.numero_identificacion.trim().len() != 13
                {
                    let respuesta_error = serde_json::json!({
                        "estado": "error",
                        "mensaje": "El formato del RFC es incorrecto",
                    });
                    return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
                }
            }
            _ => {
                if documento.numero_identificacion.trim().is_empty() {
                    let respuesta_error = serde_json::json!({
                        "estado": "error",
                        "mensaje": "El numero de identificacion no puede estar vacio",
                    });
                    return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
                }
            }
        }
    }

    Ok(())
}
