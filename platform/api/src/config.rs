use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/atomplatform".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()?;

        Ok(Config {
            database_url,
            port,
        })
    }
}

