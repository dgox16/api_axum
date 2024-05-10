use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::ubicacion_models::{
        BarrioModelo, CalleModelo, CiudadModelo, ClasificacionCiudad, DomicilioModelo,
        EstadoModelo, IndiceMarginacionBarrio, MunicipioModelo, PaisModelo, TipoCalle, TipoCiudad,
        TipoZonaBarrio,
    },
    schemas::ubicacion_schemas::{
        BuscarBarrioQuery, BuscarCalleQuery, BuscarCiudadQuery, BuscarEstadoQuery,
        BuscarMunicipioQuery, BuscarPaisQuery, CrearBarrioSchema, CrearCalleSchema,
        CrearCiudadSchema, CrearDomicilioSchema, CrearMunicipioSchema,
    },
    validators::ubicacion_validators::{
        validar_nueva_calle, validar_nueva_ciudad, validar_nueva_domicilio, validar_nuevo_municipio,
    },
    AppState,
};

pub async fn buscar_calles_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<BuscarCalleQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let limite = query.limite.unwrap_or(20);
    let palabra = query.palabra.unwrap_or(String::from("%"));
    let offset = query.offset.unwrap_or(0);

    let calles_encontradas = sqlx::query_as!(
        CalleModelo,
        r#"SELECT id_calle,nombre,tipo AS "tipo: TipoCalle" FROM calles
        WHERE nombre ILIKE '%' || $1 || '%' LIMIT $2 OFFSET $3"#,
        palabra,
        limite,
        offset
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": true,
        "datos": calles_encontradas
    });
    Ok(Json(respuesta))
}

pub async fn buscar_pais_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<BuscarPaisQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let nombre = query.nombre.unwrap_or(String::from("%"));

    let paises_encontrados = sqlx::query_as!(
        PaisModelo,
        r#"SELECT id_pais, abreviatura, nombre, orden FROM paises
        WHERE nombre ILIKE '%' || $1 || '%' ORDER BY orden"#,
        nombre,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": true,
        "datos": paises_encontrados
    });
    Ok(Json(respuesta))
}

pub async fn buscar_estado_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<BuscarEstadoQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let nombre = query.nombre.unwrap_or(String::from("%"));

    let estados_encontrados = sqlx::query_as!(
        EstadoModelo,
        r#"SELECT id_estado, nombre, abreviado, corto, clave_buro FROM estados_mexico
        WHERE nombre ILIKE '%' || $1 || '%' ORDER BY nombre"#,
        nombre,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": true,
        "datos": estados_encontrados
    });
    Ok(Json(respuesta))
}

pub async fn crear_nueva_calle_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearCalleSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_calle(&body)?;
    let nueva_calle = sqlx::query_as!(
        CalleModelo,
        r#"INSERT INTO calles (nombre, tipo) VALUES ($1, $2)
        RETURNING id_calle,nombre,tipo AS "tipo: TipoCalle""#,
        body.nombre.to_string(),
        body.tipo as TipoCalle
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": true,
        "datos": nueva_calle
    });
    Ok(Json(respuesta))
}

pub async fn crear_nuevo_domicilio_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearDomicilioSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_domicilio(&body)?;
    let nuevo_domicilio = sqlx::query_as!(
        DomicilioModelo,
        "INSERT INTO domicilios
        (cp,colonia,calle_id,entre_calle_id,y_calle_id,
        numero_exterior,numero_interior,geolocalizacion)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *",
        body.cp.to_string(),
        body.colonia.to_string(),
        body.calle_id,
        body.entre_calle_id,
        body.y_calle_id,
        body.numero_exterior.to_string(),
        body.numero_interior,
        body.geolocalizacion
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": true,
        "datos": nuevo_domicilio
    });
    Ok(Json(respuesta))
}

pub async fn crear_nuevo_municipio_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearMunicipioSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nuevo_municipio(&body)?;
    let nuevo_municipio = sqlx::query_as!(
        MunicipioModelo,
        "INSERT INTO municipios
        (nombre, estado, factor_riesgo) 
        VALUES ($1,$2,$3) 
        RETURNING id_municipio, nombre, estado, factor_riesgo",
        body.nombre,
        body.estado,
        body.factor_riesgo
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = serde_json::json!({
        "estado": true,
        "datos": nuevo_municipio
    });

    Ok(Json(respuesta))
}

pub async fn buscar_municipios_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<BuscarMunicipioQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let nombre = query.nombre.unwrap_or(String::from("%"));
    let limite = query.limite.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    let municipios_encontrados = sqlx::query_as!(
        MunicipioModelo,
        "SELECT id_municipio, nombre, estado, factor_riesgo
        FROM municipios 
        WHERE nombre ILIKE '%' || $1 || '%'
        LIMIT $2 OFFSET $3",
        nombre,
        limite,
        offset
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = serde_json::json!({
        "estado": true,
        "datos": municipios_encontrados
    });

    Ok(Json(respuesta))
}

pub async fn crear_nueva_ciudad_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearCiudadSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_ciudad(&body)?;
    let nueva_ciudad = sqlx::query_as!(
        CiudadModelo,
        r#"INSERT INTO ciudades
        (clave_localidad, estado, municipio, nombre, tipo,
        clasificacion, numero_habitantes, orden, cp) 
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) 
        RETURNING id_ciudad, clave_localidad, estado,
        municipio, nombre, tipo AS "tipo: TipoCiudad",
        clasificacion AS "clasificacion: ClasificacionCiudad",
        numero_habitantes,orden,cp"#,
        body.clave_localidad,
        body.estado,
        body.municipio,
        body.nombre,
        body.tipo as TipoCiudad,
        body.clasificacion as ClasificacionCiudad,
        body.numero_habitantes,
        body.orden,
        body.cp
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = serde_json::json!({
        "estado": true,
        "datos": nueva_ciudad
    });

    Ok(Json(respuesta))
}

pub async fn buscar_ciudades_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<BuscarCiudadQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let nombre = query.nombre.unwrap_or(String::from("%"));
    let limite = query.limite.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let ciudades_encontradas = sqlx::query_as!(
        CiudadModelo,
        r#"SELECT id_ciudad, clave_localidad, estado,
        municipio, nombre, tipo AS "tipo: TipoCiudad",
        clasificacion AS "clasificacion: ClasificacionCiudad",
        numero_habitantes,orden,cp FROM ciudades
        WHERE nombre ILIKE '%' || $1 || '%'
        LIMIT $2 OFFSET $3"#,
        nombre,
        limite,
        offset
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = serde_json::json!({
        "estado": true,
        "datos": ciudades_encontradas
    });

    Ok(Json(respuesta))
}

pub async fn crear_nuevo_barrio_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearBarrioSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let nuevo_barrio = sqlx::query_as!(
        BarrioModelo,
        r#"INSERT INTO barrios
        (ciudad, nombre, cp, tipo_cp, sindicatura, tipo_zona, numero_habitantes,
        indice_marginacion, ponderacion_5c, c_municipio, unico_asentamiento) 
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) 
        RETURNING id_barrio, ciudad, nombre, cp, tipo_cp, sindicatura,
        tipo_zona AS "tipo_zona: TipoZonaBarrio", numero_habitantes,
        indice_marginacion AS "indice_marginacion: IndiceMarginacionBarrio",
        ponderacion_5c, c_municipio, unico_asentamiento"#,
        body.ciudad,
        body.nombre,
        body.cp,
        body.tipo_cp,
        body.sindicatura,
        body.tipo_zona as TipoZonaBarrio,
        body.numero_habitantes,
        body.indice_marginacion as IndiceMarginacionBarrio,
        body.ponderacion_5c,
        body.c_municipio,
        body.unico_asentamiento
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = serde_json::json!({
        "estado": true,
        "datos": nuevo_barrio
    });

    Ok(Json(respuesta))
}

pub async fn buscar_barrios_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<BuscarBarrioQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let nombre = query.nombre.unwrap_or(String::from("%"));
    let limite = query.limite.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let barrios_encontrados = sqlx::query_as!(
        BarrioModelo,
        r#"SELECT id_barrio, ciudad, nombre, cp, tipo_cp, sindicatura,
        tipo_zona AS "tipo_zona: TipoZonaBarrio", numero_habitantes,
        indice_marginacion AS "indice_marginacion: IndiceMarginacionBarrio",
        ponderacion_5c, c_municipio, unico_asentamiento
        FROM barrios WHERE nombre ILIKE '%' || $1 || '%'
        LIMIT $2 OFFSET $3"#,
        nombre,
        limite,
        offset
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;
    let respuesta = serde_json::json!({
        "estado": true,
        "datos": barrios_encontrados
    });
    Ok(Json(respuesta))
}
