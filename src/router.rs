use std::sync::Arc;

use axum::Router;

use crate::{
    routes::{
        auth_routes::auth_router, domicilio_routes::domicilio_router, poliza_routes::poliza_router,
        sucursal_routes::sucursal_router, test_routes::test_router,
    },
    AppState,
};

// Creamos un enrutador en un archivo aparte
pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(auth_router(app_state.clone()))
        .merge(domicilio_router(app_state.clone()))
        .merge(sucursal_router(app_state.clone()))
        .merge(poliza_router(app_state.clone()))
        .merge(test_router())
}
