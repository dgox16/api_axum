#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secreto: String,
    pub jwt_expira_en: i64,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL debe ser asignado");
        let jwt_secreto = std::env::var("JWT_SECRETO").expect("JWT_SECRETO debe ser asignado");
        let jwt_expira_en =
            std::env::var("JWT_EXPIRA_EN").expect("JWT_EXPIRA_EN debe ser asignado");
        Config {
            database_url,
            jwt_secreto,
            jwt_expira_en: jwt_expira_en
                .parse::<i64>()
                .expect("Fallo al parsear el valor de JWT_EXPIRA_EN"),
        }
    }
}
