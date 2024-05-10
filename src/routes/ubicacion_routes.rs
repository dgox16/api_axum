use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::ubicacion_handlers::{
        buscar_barrios_handler, buscar_calles_handler, buscar_ciudades_handler,
        buscar_estado_handler, buscar_pais_handler, crear_nueva_calle_handler,
        crear_nueva_ciudad_handler, crear_nuevo_barrio_handler, crear_nuevo_domicilio_handler,
        crear_nuevo_municipio_handler,
    },
    AppState,
};

pub fn ubicacion_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/calle/nueva", post(crear_nueva_calle_handler))
        .route("/api/calle/buscar/", get(buscar_calles_handler))
        .route("/api/pais/buscar/", get(buscar_pais_handler))
        .route("/api/estado_mexico/buscar/", get(buscar_estado_handler))
        .route("/api/domicilio/nuevo", post(crear_nuevo_domicilio_handler))
        .route("/api/municipio/nuevo", post(crear_nuevo_municipio_handler))
        .route("/api/ciudad/nueva", post(crear_nueva_ciudad_handler))
        .route("/api/ciudad/buscar/", get(buscar_ciudades_handler))
        .route("/api/barrio/nuevo", post(crear_nuevo_barrio_handler))
        .route("/api/barrio/buscar/", get(buscar_barrios_handler))
        .with_state(app_state)
}
