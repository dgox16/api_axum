use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::{Query, State},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::TimeDelta;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand_core::OsRng;
use serde_json::json;

use crate::{
    models::{
        token_models::TokenClaims,
        user_models::{PropositoJWT, UsuarioModelo},
    },
    responses::{error_responses::error_base_datos, user_responses::UsuarioFormateado},
    schemas::auth_schemas::{
        BuscarUsuarioQuery, InicioSesionUsuarioSchema, RefrescarTokenSchema, RegistroUsuarioSchema,
    },
    AppState,
};

// Funcion para no mostrar la contraseña en ningun momento
pub async fn formatear_usuario(
    user: &UsuarioModelo,
    State(data): State<Arc<AppState>>,
) -> Result<UsuarioFormateado, (StatusCode, Json<serde_json::Value>)> {
    // Consulta SQL para obtener el nombre del rol
    let rol = sqlx::query!("SELECT nombre FROM roles WHERE id = $1", user.id_rol)
        .fetch_one(&data.db)
        .await
        .map_err(error_base_datos)?;

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
    let usuario_existe = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM usuarios WHERE email = $1 OR usuario = $2)",
        body.email.to_ascii_lowercase(),
        body.usuario.to_owned()
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    // En caso de no exisir devolvemos un json de fallo
    if let Some(existe) = usuario_existe {
        if existe {
            let respuesta_error = serde_json::json!({
                "estado": false,
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
                "estado": false,
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
    .map_err(error_base_datos)?;

    let usuario_formateado = formatear_usuario(&nuevo_usuario, axum::extract::State(data)).await?;

    let respuesta = json!({
        "estado": true,
        "datos": usuario_formateado
    });

    Ok(Json(respuesta))
}

pub fn crear_token_jwt(
    tiempo_vida: TimeDelta,
    sub: i32,
    proposito: PropositoJWT,
    secret: &String,
) -> String {
    let fecha_actual = chrono::Utc::now();
    let iat = fecha_actual.timestamp() as usize;
    let exp = (fecha_actual + tiempo_vida).timestamp() as usize;

    let claims: TokenClaims = TokenClaims {
        sub,
        exp,
        iat,
        proposito,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
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
    .map_err(error_base_datos)?
    // Manejamos el posible fallo al no encontrar un usuario con ese email
    .ok_or_else(|| {
        let respuesta_error = serde_json::json!({
            "estado": false,
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
            "estado": false,
            "mensaje": "Invalido usuario o contraseña",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    let access_exp = chrono::Duration::minutes(data.env.jwt_expira_en);
    let refresh_exp = chrono::Duration::days(7);

    let secreto = data.env.jwt_secreto.clone();

    let access_token = crear_token_jwt(
        access_exp,
        usuario_encontrado.id,
        PropositoJWT::AccessToken,
        &secreto,
    );

    let refresh_token = crear_token_jwt(
        refresh_exp,
        usuario_encontrado.id,
        PropositoJWT::RefreshToken,
        &secreto,
    );

    let respuesta = json!({
        "estado": true,
        "datos": {
            "access_token": access_token,
            "refresh_token": refresh_token
    }
    });

    Ok(Json(respuesta))
}

pub async fn refrescar_token_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RefrescarTokenSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let claims = decode::<TokenClaims>(
        &body.refresh_token,
        &DecodingKey::from_secret(data.env.jwt_secreto.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "estado": false,
                "mensaje": format!("Token Invalido"),
            })),
        )
    })?
    .claims;

    let access_exp = chrono::Duration::minutes(data.env.jwt_expira_en);
    let refresh_exp = chrono::Duration::days(7);

    let secreto = data.env.jwt_secreto.clone();

    let access_token = crear_token_jwt(access_exp, claims.sub, PropositoJWT::AccessToken, &secreto);

    let refresh_token = crear_token_jwt(
        refresh_exp,
        claims.sub,
        PropositoJWT::RefreshToken,
        &secreto,
    );

    let respuesta = json!({
        "estado": true,
        "datos": {
            "access_token": access_token,
            "refresh_token": refresh_token
    }
    });

    Ok(Json(respuesta))
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
    let mut respuesta = Response::new(
        json!({"estado": true, "mensaje": "Se cerro session de forma correcta"}).to_string(),
    );
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
    let usuario_formateado = formatear_usuario(&usuario, axum::extract::State(data)).await?;
    let respuesta = json!({
        "estado": true,
        "datos": usuario_formateado
    });

    // Devolvemos la respuesta HTTP con el JSON
    Ok(Json(respuesta))
}

pub async fn buscar_usuarios_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<BuscarUsuarioQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let limite = query.limite.unwrap_or(20);
    let palabra = query.palabra.unwrap_or(String::from("%"));
    let offset = query.offset.unwrap_or(0);

    let usuarios_encontrados = sqlx::query_as!(
        UsuarioModelo,
        r#"SELECT * FROM usuarios
        WHERE usuario ILIKE '%' || $1 || '%' LIMIT $2 OFFSET $3"#,
        palabra,
        limite,
        offset
    )
    .fetch_all(&data.db)
    .await
    .map_err(error_base_datos)?;

    let mut usuarios_formateados: Vec<UsuarioFormateado> = vec![];

    for usuario_encontrado in usuarios_encontrados {
        let usuario_formateado =
            formatear_usuario(&usuario_encontrado, axum::extract::State(data.clone())).await?;
        usuarios_formateados.push(usuario_formateado);
    }

    let respuesta = json!({
        "estado": true,
        "datos": usuarios_formateados
    });
    Ok(Json(respuesta))
}
