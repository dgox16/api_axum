use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    handlers::contrato_captacion_handlers::{
        crear_contrato_captacion_handler, listar_contratos_captacion_handler,
    },
    middlewares::jwt_middlewares::auth_required,
    AppState,
};

pub fn contrato_captacion_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/contrato_captacion/nuevo/",
            post(crear_contrato_captacion_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_required,
            )),
        )
        .route(
            "/api/contrato_captacion/listar/",
            get(listar_contratos_captacion_handler),
        )
        .with_state(app_state)
}
