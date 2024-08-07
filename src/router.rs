use std::sync::Arc;

use axum::Router;

use crate::{
    routes::{
        auth_routes::auth_router, contratos_captacion_routes::contrato_captacion_router,
        entidades_routes::entidades_router, persona_routes::persona_router,
        poliza_routes::poliza_router, reportes_routes::reportes_router, test_routes::test_router,
        trabajo_routes::trabajo_router, ubicacion_routes::ubicacion_router,
    },
    AppState,
};

// Creamos un enrutador en un archivo aparte
pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(auth_router(app_state.clone()))
        .merge(ubicacion_router(app_state.clone()))
        .merge(entidades_router(app_state.clone()))
        .merge(poliza_router(app_state.clone()))
        .merge(persona_router(app_state.clone()))
        .merge(trabajo_router(app_state.clone()))
        .merge(contrato_captacion_router(app_state.clone()))
        .merge(reportes_router(app_state.clone()))
        .merge(test_router())
}
