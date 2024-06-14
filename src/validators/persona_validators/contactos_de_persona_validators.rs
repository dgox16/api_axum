use axum::{http::StatusCode, Json};

use crate::{
    models::persona_models::contactos_de_persona_models::TipoContacto,
    schemas::persona_schemas::contactos_de_persona_schemas::CrearContactoDePersonaSchema,
};

pub fn validar_nuevo_contacto_de_persona(
    contactos_de_persona: &Vec<CrearContactoDePersonaSchema>,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    for contacto in contactos_de_persona {
        match contacto.tipo {
            TipoContacto::TelefonoFijo => {
                if contacto.contacto.trim().is_empty() || contacto.contacto.trim().len() != 10 {
                    let respuesta_error = serde_json::json!({
                        "estado": false,
                        "mensaje": "El formato del telefono es incorrecto",
                    });
                    return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
                }
            }
            TipoContacto::TelefonoMovil => {
                if contacto.contacto.trim().is_empty() || contacto.contacto.trim().len() != 10 {
                    let respuesta_error = serde_json::json!({
                        "estado": false,
                        "mensaje": "El formato del telefono es incorrecto",
                    });
                    return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
                }
            }
            _ => {
                if contacto.contacto.trim().is_empty() {
                    let respuesta_error = serde_json::json!({
                        "estado": false,
                        "mensaje": "El contacto no puede estar vacio",
                    });
                    return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
                }
            }
        }
    }

    Ok(())
}
