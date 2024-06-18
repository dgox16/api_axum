use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    handlers::contrato_captacion_handlers::contrato_captacion_principal_handlers::{
        abono_contrato_captacion_handler, cargo_contrato_captacion_handler,
        crear_contrato_captacion_handler, listar_contratos_captacion_handler,
        obtener_saldo_contrato_captacion_handler,
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
        .route(
            "/api/contrato_captacion/obtener_saldo/",
            get(obtener_saldo_contrato_captacion_handler),
        )
        .route(
            "/api/contrato_captacion/abono/",
            post(abono_contrato_captacion_handler),
        )
        .route(
            "/api/contrato_captacion/cargo/",
            post(cargo_contrato_captacion_handler),
        )
        .with_state(app_state)
}
