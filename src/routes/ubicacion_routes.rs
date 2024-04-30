use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::ubicacion_handlers::{
        buscar_calles_handler, crear_nueva_calle_handler, crear_nuevo_domicilio_handler,
    },
    AppState,
};

pub fn ubicacion_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/calle/nueva", post(crear_nueva_calle_handler))
        .route("/api/calle/buscar/", get(buscar_calles_handler))
        .route("/api/domicilio/nuevo", post(crear_nuevo_domicilio_handler))
        .with_state(app_state)
}
