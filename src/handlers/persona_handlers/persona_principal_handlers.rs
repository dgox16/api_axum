use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;

use crate::{
    models::{
        persona_models::{
            persona_principal_models::PersonaModelo,
            persona_types::{EstadoCivilPersona, SexoPersona, ViviendaPersona},
        },
        user_models::UsuarioModelo,
    },
    schemas::persona_schemas::persona_principal_schemas::{
        BuscarPersonaQuery, CrearPersonaSchema, ObtenerPersonaParams,
    },
    validators::persona_validators::persona_principal_validators::validar_nueva_persona,
    AppState,
};

use super::{
    persona_aspirante_handlers::obtener_persona_aspirante_handler,
    persona_aval_handlers::obtener_persona_aval_handler,
    persona_socio_handlers::obtener_persona_socio_handler,
};

pub async fn crear_nueva_persona_handler(
    State(data): State<Arc<AppState>>,
    Extension(usuario): Extension<UsuarioModelo>,
    Json(body): Json<CrearPersonaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_persona(&body)?;
    let nueva_persona = sqlx::query_as!(
        PersonaModelo,
        r#"INSERT INTO personas 
        (nombre,apellido_paterno,apellido_materno,tipo,sexo,
        usuario_actualizo,cp,barrio,ciudad,calle,numero_exterior,
        numero_interior,vivienda,geolocalizacion,
        observaciones_geolocalizacion,fecha_nacimiento,
        pais_nacimiento,estado_civil,persona_conyuge,
        es_embargo_precautorio,bloqueado_autoridad,tercero_autorizado)
        VALUES ($1, $2, $3, $4, $5, $6,$7,$8,$9,$10,$11,$12,$13,
        $14,$15,$16,$17,$18,$19,$20,$21,$22) RETURNING
        id_persona,nombre,apellido_paterno,apellido_materno,tipo,
        sexo AS "sexo: SexoPersona",fecha_actualizacion,
        usuario_actualizo,cp,barrio,ciudad,calle,numero_exterior,
        numero_interior,vivienda AS "vivienda: ViviendaPersona",
        geolocalizacion,observaciones_geolocalizacion,
        fecha_nacimiento,pais_nacimiento,
        estado_civil AS "estado_civil: EstadoCivilPersona",
        persona_conyuge,es_embargo_precautorio,bloqueado_autoridad,tercero_autorizado"#,
        body.nombre,
        body.apellido_paterno,
        body.apellido_materno,
        body.tipo,
        body.sexo as SexoPersona,
        usuario.id,
        body.cp,
        body.barrio,
        body.ciudad,
        body.calle,
        body.numero_exterior,
        body.numero_interior,
        body.vivienda as ViviendaPersona,
        body.geolocalizacion,
        body.observaciones_geolocalizacion,
        body.fecha_nacimiento,
        body.pais_nacimiento,
        body.estado_civil as EstadoCivilPersona,
        body.persona_conyuge,
        body.es_embargo_precautorio,
        body.bloqueado_autoridad,
        body.tercero_autorizado
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

    let respuesta = json!({
        "estado": true,
        "datos": nueva_persona
    });

    Ok(Json(respuesta))
}

pub async fn obtener_persona_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let persona_encontrada = sqlx::query_as!(
        PersonaModelo,
        r#"SELECT id_persona,nombre,apellido_paterno,apellido_materno,tipo,
        sexo AS "sexo: SexoPersona",fecha_actualizacion,
        usuario_actualizo,cp,barrio,ciudad,calle,numero_exterior,
        numero_interior,vivienda AS "vivienda: ViviendaPersona",
        geolocalizacion,observaciones_geolocalizacion,
        fecha_nacimiento,pais_nacimiento,
        estado_civil AS "estado_civil: EstadoCivilPersona",
        persona_conyuge,es_embargo_precautorio,bloqueado_autoridad,tercero_autorizado
        FROM personas WHERE id_persona=$1"#,
        params.id_persona
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

    let mut respuesta = json!({
        "estado": true,
        "datos": {
            "persona": persona_encontrada
        }
    });

    match persona_encontrada.tipo {
        1 => {
            let aspirante =
                obtener_persona_aspirante_handler(&data, persona_encontrada.id_persona).await?;
            respuesta["datos"]["datos_aspirante"] = json!(aspirante);
        }
        2 => {
            let socio = obtener_persona_socio_handler(&data, persona_encontrada.id_persona).await?;
            respuesta["datos"]["datos_socio"] = json!(socio);
        }
        3 => {
            let aval = obtener_persona_aval_handler(&data, persona_encontrada.id_persona).await?;
            respuesta["datos"]["datos_aval"] = json!(aval);
        }
        _ => {}
    }

    Ok(Json(respuesta))
}

pub async fn buscar_personas_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<BuscarPersonaQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let palabra_clave = query.palabra_clave.unwrap_or(String::from("%"));
    let limite = query.limite.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    match query.tipo {
        Some(tipo) => {
            let tipo_bd = match tipo.as_str() {
                "aspirante" => 1,
                "socio" => 2,
                "aval_externo" => 3,
                "menor" => 4,
                "conyuge" => 5,
                "cliente" => 6,
                "sucursal" => 7,
                "tercero_autorizado" => 8,
                "propietario_real" => 9,
                "proveedor_de_recursos" => 10,
                "tutor" => 11,
                "referencia" => 12,
                "beneficiario" => 13,
                _ => 0,
            };

            if tipo_bd == 0 {
                let respuesta_error = serde_json::json!({
                    "estado": "error",
                    "mensaje": "El tipo de persona es invalido",
                });
                return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
            }

            let personas_encontradas = sqlx::query_as!(
                PersonaModelo,
                r#"SELECT id_persona,nombre,apellido_paterno,apellido_materno,tipo,
                sexo AS "sexo: SexoPersona",fecha_actualizacion,
                usuario_actualizo,cp,barrio,ciudad,calle,numero_exterior,
                numero_interior,vivienda AS "vivienda: ViviendaPersona",
                geolocalizacion,observaciones_geolocalizacion,
                fecha_nacimiento,pais_nacimiento,
                estado_civil AS "estado_civil: EstadoCivilPersona",
                persona_conyuge,es_embargo_precautorio,bloqueado_autoridad,tercero_autorizado
                FROM personas WHERE 
                (nombre ILIKE '%' || $1 || '%'
                OR apellido_paterno ILIKE '%' || $1 || '%'
                OR apellido_materno ILIKE '%' || $1 || '%')
                AND tipo=$2
                LIMIT $3 OFFSET $4"#,
                palabra_clave,
                tipo_bd,
                limite,
                offset
            )
            .fetch_all(&data.db)
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

            let respuesta = json!({
                "estado": true,
                "datos": personas_encontradas
            });

            Ok(Json(respuesta))
        }
        None => {
            let personas_encontradas = sqlx::query_as!(
                PersonaModelo,
                r#"SELECT id_persona,nombre,apellido_paterno,apellido_materno,tipo,
                sexo AS "sexo: SexoPersona",fecha_actualizacion,
                usuario_actualizo,cp,barrio,ciudad,calle,numero_exterior,
                numero_interior,vivienda AS "vivienda: ViviendaPersona",
                geolocalizacion,observaciones_geolocalizacion,
                fecha_nacimiento,pais_nacimiento,
                estado_civil AS "estado_civil: EstadoCivilPersona",
                persona_conyuge,es_embargo_precautorio,bloqueado_autoridad,tercero_autorizado
                FROM personas WHERE nombre ILIKE '%' || $1 || '%'
                OR apellido_paterno ILIKE '%' || $1 || '%'
                OR apellido_materno ILIKE '%' || $1 || '%'
                LIMIT $2 OFFSET $3"#,
                palabra_clave,
                limite,
                offset
            )
            .fetch_all(&data.db)
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

            let respuesta = json!({
                "estado": true,
                "datos": personas_encontradas
            });

            Ok(Json(respuesta))
        }
    }
}
