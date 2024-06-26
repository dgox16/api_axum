use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    handlers::persona_handlers::{
        contactos_de_persona_handlers::crear_nuevo_contacto_de_persona_handler,
        documentos_de_persona_handlers::crear_nuevo_documento_de_persona_handler,
        persona_aspirante_handlers::crear_nueva_persona_aspirante_handler,
        persona_aval_handlers::crear_nueva_persona_aval_handler,
        persona_beneficiario_handlers::crear_nueva_persona_beneficiario_handler,
        persona_cliente_handlers::crear_nueva_persona_cliente_handler,
        persona_conyuge_handlers::crear_nueva_persona_conyuge_handler,
        persona_menor_handlers::crear_nueva_persona_menor_handler,
        persona_principal_handlers::{
            buscar_personas_handler, crear_nueva_persona_handler, obtener_persona_handler,
        },
        persona_propietario_real_handlers::crear_nueva_persona_propietario_real_handler,
        persona_proveedor_recursos_handlers::crear_nueva_persona_proveedor_recursos_handler,
        persona_socio_handlers::crear_nueva_persona_socio_handler,
        persona_sucursal_handlers::crear_nueva_persona_sucursal_handler,
        persona_tercero_autorizado_handlers::crear_nueva_persona_tercero_autorizado_handler,
        persona_tutor_handlers::crear_nueva_persona_tutor_handler,
        relaciones_de_persona_handlers::crear_nueva_relacion_de_persona_handler,
        tutores_de_persona_handlers::crear_nuevo_tutor_de_persona_handler,
    },
    middlewares::jwt_middlewares::auth_required,
    AppState,
};

pub fn persona_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/persona/nueva",
            post(crear_nueva_persona_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_required,
            )),
        )
        .route(
            "/api/persona/obtener/:id_persona",
            get(obtener_persona_handler),
        )
        .route("/api/persona/buscar/", get(buscar_personas_handler))
        .route(
            "/api/persona/aspirante/nuevo/:id_persona",
            post(crear_nueva_persona_aspirante_handler),
        )
        .route(
            "/api/persona/socio/nuevo/:id_persona",
            post(crear_nueva_persona_socio_handler),
        )
        .route(
            "/api/persona/aval/nuevo/:id_persona",
            post(crear_nueva_persona_aval_handler),
        )
        .route(
            "/api/persona/menor/nuevo/:id_persona",
            post(crear_nueva_persona_menor_handler),
        )
        .route(
            "/api/persona/conyuge/nuevo/:id_persona",
            post(crear_nueva_persona_conyuge_handler),
        )
        .route(
            "/api/persona/cliente/nuevo/:id_persona",
            post(crear_nueva_persona_cliente_handler),
        )
        .route(
            "/api/persona/sucursal/nuevo/:id_persona",
            post(crear_nueva_persona_sucursal_handler),
        )
        .route(
            "/api/persona/tercero_autorizado/nuevo/:id_persona",
            post(crear_nueva_persona_tercero_autorizado_handler),
        )
        .route(
            "/api/persona/propietario_real/nuevo/:id_persona",
            post(crear_nueva_persona_propietario_real_handler),
        )
        .route(
            "/api/persona/proveedor_recursos/nuevo/:id_persona",
            post(crear_nueva_persona_proveedor_recursos_handler),
        )
        .route(
            "/api/persona/tutor/nuevo/:id_persona",
            post(crear_nueva_persona_tutor_handler),
        )
        .route(
            "/api/persona/beneficiario/nuevo/:id_persona",
            post(crear_nueva_persona_beneficiario_handler),
        )
        .route(
            "/api/persona/contacto_persona/nuevo/:id_persona",
            post(crear_nuevo_contacto_de_persona_handler),
        )
        .route(
            "/api/persona/documento_persona/nuevo/:id_persona",
            post(crear_nuevo_documento_de_persona_handler),
        )
        .route(
            "/api/persona/relacion_persona/nuevo/:id_persona",
            post(crear_nueva_relacion_de_persona_handler),
        )
        .route(
            "/api/persona/tutor_persona/nuevo/:id_persona",
            post(crear_nuevo_tutor_de_persona_handler),
        )
        .with_state(app_state)
}
