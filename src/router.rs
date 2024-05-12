use std::sync::Arc;

use axum::Router;

use crate::{
    routes::{
        auth_routes::auth_router, entidades_routes::entidades_router,
        persona_routes::persona_router, poliza_routes::poliza_router, test_routes::test_router,
        ubicacion_routes::ubicacion_router,
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
        .merge(test_router())
}
