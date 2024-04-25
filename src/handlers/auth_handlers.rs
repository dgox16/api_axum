use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde_json::json;

use crate::{
    models::{token_models::TokenClaims, user_models::UsuarioModelo},
    responses::user_responses::UsuarioFormateado,
    schemas::auth_schemas::{InicioSesionUsuarioSchema, RegistroUsuarioSchema},
    AppState,
};

// Funcion para no mostrar la contraseña en ningun momento
pub async fn formatear_usuario(
    user: &UsuarioModelo,
    State(data): State<Arc<AppState>>,
) -> Result<UsuarioFormateado, sqlx::Error> {
    // Consulta SQL para obtener el nombre del rol
    let rol = sqlx::query!("SELECT nombre FROM roles WHERE id = $1", user.id_rol)
        .fetch_one(&data.db)
        .await?;

    Ok(UsuarioFormateado {
        id: user.id.to_string(),
        email: user.email.clone(),
        usuario: user.usuario.clone(),
        rol: rol.nombre,
        createdAt: user.created_at.unwrap(),
        updatedAt: user.updated_at.unwrap(),
    })
}

// Funcion para el registro de usuarios
pub async fn registrar_usuario_handler(
    State(data): State<Arc<AppState>>, // Necesitamos el estado global
    Json(body): Json<RegistroUsuarioSchema>, // El body formareado de la request
                                       // Devolvemos un resultado con el codigo de estado y un json
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Vemos si el usuario existe buscandolo en la base de datos
    let usuario_existe: Option<bool> = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM usuarios WHERE email = $1 OR usuario = $2)",
    )
    .bind(body.email.to_owned().to_ascii_lowercase())
    .bind(body.usuario.to_owned())
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        // En caso de fallar la conexion devolvemos un json de fallo
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    // En caso de no exisir devolvemos un json de fallo
    if let Some(existe) = usuario_existe {
        if existe {
            let respuesta_error = serde_json::json!({
                "estado": "error",
                "mensaje": "Ya existe el email y/o el usuario",
            });
            return Err((StatusCode::CONFLICT, Json(respuesta_error)));
        }
    }
    // Creamos una semilla para el hash
    let salt = SaltString::generate(&mut OsRng);
    // Encriptamos la contraseña
    let contraseña_encriptada = Argon2::default()
        .hash_password(body.contraseña.as_bytes(), &salt)
        .map_err(|e| {
            // Manejamos el posible error al hashear
            let respuesta_error = serde_json::json!({
                "estado": "error",
                "message": format!("Fallo al encriptar la contraseña: {}", e),
            });
            (StatusCode::CONFLICT, Json(respuesta_error))
        })
        .map(|hash| hash.to_string())?;

    // Creamos el usuario en la base de datos
    let nuevo_usuario = sqlx::query_as!(
        UsuarioModelo,
        "INSERT INTO usuarios (usuario,email,contraseña) VALUES ($1, $2, $3) RETURNING *",
        body.usuario.to_string(),
        body.email.to_string().to_ascii_lowercase(),
        contraseña_encriptada
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        // Manejamos el posible fallo con la base de datos
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let usuario_formateado = formatear_usuario(&nuevo_usuario, axum::extract::State(data)).await;
    match usuario_formateado {
        Ok(usuario) => {
            let respuesta = json!({
                "estado": "exitoso",
                "data": usuario
            });

            // Devolvemos la respuesta HTTP con el JSON
            Ok(Json(respuesta))
        }
        Err(e) => {
            // Si ocurrió un error al obtener la información del usuario, devolvemos un error 500
            let respuesta_error = serde_json::json!({
                "estado": "error",
                "mensaje": format!("Fallo con el nuevo usuario: {}", e),
            });

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error)))
        }
    }
}

// Funcion para el login de usuarios
pub async fn inicio_sesion_handler(
    State(data): State<Arc<AppState>>, // Necesitamos el estado global
    Json(body): Json<InicioSesionUsuarioSchema>, // El body formareado de la request
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Buscamos un usuario con el username del body
    let usuario_encontrado = sqlx::query_as!(
        UsuarioModelo,
        "SELECT * FROM usuarios WHERE usuario = $1",
        body.usuario.to_ascii_lowercase()
    )
    .fetch_optional(&data.db)
    .await
    // Manejamos el posible error en la base de datos
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?
    // Manejamos el posible fallo al no encontrar un usuario con ese email
    .ok_or_else(|| {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "Invalido usuario o contraseña",
        });
        (StatusCode::BAD_REQUEST, Json(respuesta_error))
    })?;

    // Validamos que la contraseña mandada sea igual a la de la base de datos
    let es_contraseña_valida = match PasswordHash::new(&usuario_encontrado.contraseña) {
        Ok(contraseña_parseada) => Argon2::default()
            .verify_password(body.contraseña.as_bytes(), &contraseña_parseada)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    // Si la validadacion no es correcta; devolvemos el fallo de que la contraseña es invalida
    if !es_contraseña_valida {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "Invalido usuario o contraseña",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    // Usamos la fecha actual para el token
    let fecha_actual = chrono::Utc::now();
    let iat = fecha_actual.timestamp() as usize;
    // El token sera valido por 60 min
    let exp =
        (fecha_actual + chrono::Duration::minutes(data.env.jwt_expira_en)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: usuario_encontrado.id.to_string(), // Guardamos el usuario tambien en el token
        exp,
        iat,
    };

    // Creamos el token con los datos anteriores y la secret
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secreto.as_ref()),
    )
    .unwrap();

    // Crearemos una cookie con el token que durara una hora
    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::minutes(data.env.jwt_expira_en))
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true);

    // Devolvemos exito si no fallo y devolvemos el token
    let mut respuesta = Response::new(json!({"estado": "exitoso", "data": token}).to_string());
    respuesta
        .headers_mut()
        // Insertamos la cookie en el cliente
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(respuesta)
}

// Crearemos una funcion para el logout que no tiene parametros y devolvera un resultado con el codigo y json
pub async fn cerrar_sesion_handler(
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Para cerrar sesion mandamos un cookie para que sobreescriba al token pero vacio y con duracion negativa
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::None)
        .http_only(true);

    // Devolvemos e insertamos al cliente la cookie vacia
    let mut respuesta = Response::new(json!({"estado": "exitoso"}).to_string());
    respuesta
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(respuesta)
}

// Esta funcion es para mostrar una vista de perfil protegida
pub async fn obtener_usuario_actual_handler(
    State(data): State<Arc<AppState>>, // Necesitamos el estado global
    Extension(usuario): Extension<UsuarioModelo>, // En la extension se encuentra el usuario gracias al middleware de autentificacion
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Simplemente devolvemos al usuario con los datos filtrados
    let usuario_formateado = formatear_usuario(&usuario, axum::extract::State(data)).await;
    match usuario_formateado {
        Ok(usuario) => {
            let respuesta = json!({
                "estado": "exitoso",
                "data": usuario
            });

            // Devolvemos la respuesta HTTP con el JSON
            Ok(Json(respuesta))
        }
        Err(e) => {
            // Si ocurrió un error al obtener la información del usuario, devolvemos un error 500
            let respuesta_error = serde_json::json!({
                "estado": "error",
                "mensaje": format!("Fallo con el usuario: {}", e),
            });

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error)))
        }
    }
}
