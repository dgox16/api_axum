use axum::{http::StatusCode, Json};

use crate::schemas::persona_schemas::contactos_de_persona_schemas::CrearContactoDePersonaSchema;

pub fn validar_nuevo_contacto_de_persona(
    contactos_de_persona: &Vec<CrearContactoDePersonaSchema>,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    for contacto in contactos_de_persona {
        if contacto.contacto.trim().is_empty() {
            let respuesta_error = serde_json::json!({
                "estado": "error",
                "mensaje": "El contacto no puede estar vacio",
            });
            return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
        }
    }

    Ok(())
}
