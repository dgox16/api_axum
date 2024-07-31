use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::poliza_handlers::{
        poliza_delete_handlers::{eliminar_detalle_poliza_handler, eliminar_poliza_handler},
        poliza_insert_handlers::{crear_detalle_poliza_handler, crear_nueva_poliza_handler},
        poliza_put_handlers::editar_poliza_handler,
        poliza_select_handlers::{
            buscar_detalles_handler, buscar_polizas_handler, obtener_poliza_handler,
        },
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
        .route("/api/poliza/buscar", get(buscar_polizas_handler))
        .route("/api/poliza/buscar_detalles", get(buscar_detalles_handler))
        .route(
            "/api/poliza/eliminar/:id_poliza",
            delete(eliminar_poliza_handler),
        )
        .route(
            "/api/poliza/obtener/:id_poliza",
            get(obtener_poliza_handler),
        )
        .route(
            "/api/poliza/editar/:id_poliza",
            put(editar_poliza_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_required,
            )),
        )
        .route(
            "/api/poliza/detalles/nuevo/:id_poliza",
            post(crear_detalle_poliza_handler),
        )
        .route(
            "/api/poliza/detalles/eliminar/:id_detalle_poliza",
            delete(eliminar_detalle_poliza_handler),
        )
        .with_state(app_state)
}
