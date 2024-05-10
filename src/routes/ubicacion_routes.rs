use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::ubicacion_handlers::{
        buscar_calles_handler, buscar_estado_handler, buscar_pais_handler,
        crear_nueva_calle_handler, crear_nueva_ciudad_handler, crear_nuevo_domicilio_handler,
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
        .route("/api/ciudad/nueva", post(crear_nueva_ciudad_handler))
        .with_state(app_state)
}
