use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{
    handlers::entidades_handlers::{
        crear_nueva_cuenta_handler, crear_nueva_empresa_handler,
        crear_nueva_frecuencia_empresa_handler, crear_nueva_sucursal_handler,
        crear_nuevo_banco_handler, crear_nuevo_proveedor_handler,
    },
    AppState,
};

pub fn entidades_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/sucursal/nueva", post(crear_nueva_sucursal_handler))
        .route("/api/banco/nuevo", post(crear_nuevo_banco_handler))
        .route("/api/cuenta/nueva", post(crear_nueva_cuenta_handler))
        .route("/api/proveedor/nuevo", post(crear_nuevo_proveedor_handler))
        .route("/api/empresa/nueva", post(crear_nueva_empresa_handler))
        .route(
            "/api/frecuencia_empresa/nueva",
            post(crear_nueva_frecuencia_empresa_handler),
        )
        .with_state(app_state)
}
