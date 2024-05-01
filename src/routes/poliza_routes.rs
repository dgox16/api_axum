use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    handlers::poliza_handlers::{
        buscar_polizas_concepto_handler, crear_nueva_poliza_handler, obtener_poliza_handler,
    },
    middlewares::jwt_middlewares::auth_required,
    AppState,
};

pub fn poliza_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/poliza/nueva",
            post(crear_nueva_poliza_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_required,
            )),
        )
        .route(
            "/api/poliza/buscar/concepto",
            get(buscar_polizas_concepto_handler),
        )
        .route(
            "/api/poliza/obtener/:id_poliza",
            get(obtener_poliza_handler),
        )
        .with_state(app_state)
}
