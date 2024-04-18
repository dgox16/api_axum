use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    handlers::auth_handlers::{
        cerrar_sesion_handler, inicio_sesion_handler, obtener_usuario_actual_handler,
        registrar_usuario_handler,
    },
    middlewares::jwt_middlewares::auth_required,
    AppState,
};

pub fn auth_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/registro", post(registrar_usuario_handler))
        .route("/api/auth/inicio_sesion", post(inicio_sesion_handler))
        .route(
            "/api/auth/cerrar_sesion",
            get(cerrar_sesion_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_required,
            )),
        )
        .route(
            "/api/usuarios/actual",
            get(obtener_usuario_actual_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_required,
            )),
        )
        .with_state(app_state)
}
