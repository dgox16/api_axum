use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::persona_models::contactos_de_persona_models::ContactoDePersonaModelo,
    models::persona_models::contactos_de_persona_models::TipoContacto,
    schemas::persona_schemas::{
        contactos_de_persona_schemas::CrearContactoDePersonaSchema,
        persona_principal_schemas::ObtenerPersonaParams,
    },
    AppState,
};

pub async fn crear_nuevo_contacto_de_persona_handlers(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<Vec<CrearContactoDePersonaSchema>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut contactos_creados = Vec::new();
    for contacto in body {
        let nuevo_contacto = sqlx::query_as!(
            ContactoDePersonaModelo,
            r#"INSERT INTO contactos_de_persona
            (id_persona, contacto, tipo, es_principal)
            VALUES ($1, $2, $3, $4)
            RETURNING 
            id_contacto_de_persona, id_persona, contacto, tipo AS "tipo: TipoContacto", es_principal"#,
            params.id_persona,
            contacto.contacto,
            contacto.tipo as TipoContacto,
            contacto.es_principal
        )
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "estado": false,
                    "mensaje": format!("Error en la base de datos: {}", e),
                })),
            )
        })?;
        contactos_creados.push(nuevo_contacto);
    }

    let respuesta = json!({
        "estado": true,
        "datos": contactos_creados
    });

    Ok(Json(respuesta))
}
