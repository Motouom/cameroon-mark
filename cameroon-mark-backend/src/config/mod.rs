use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub minio: MinioConfig,
    pub cors: CorsConfig,
    pub password_reset: PasswordResetConfig,
    pub payment: PaymentConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: u64, // in seconds
}

#[derive(Debug, Clone, Deserialize)]
pub struct MinioConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub region: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasswordResetConfig {
    pub expiration: u64, // in seconds
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaymentConfig {
    pub mtn_api_key: String,
    pub mtn_api_secret: String,
    pub orange_api_key: String,
    pub orange_api_secret: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        // Load environment variables from .env file
        dotenv().ok();
        
        // Try to load .env.local if it exists (for local development)
        dotenv::from_filename(".env.local").ok();
        
        Config {
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .expect("DATABASE_MAX_CONNECTIONS must be a number"),
            },
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .expect("SERVER_PORT must be a number"),
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
                expiration: env::var("JWT_EXPIRATION")
                    .unwrap_or_else(|_| "86400".to_string()) // 24 hours in seconds
                    .parse()
                    .expect("JWT_EXPIRATION must be a number"),
            },
            minio: MinioConfig {
                endpoint: env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT must be set"),
                access_key: env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY must be set"),
                secret_key: env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY must be set"),
                bucket: env::var("MINIO_BUCKET").expect("MINIO_BUCKET must be set"),
                region: env::var("MINIO_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            },
            cors: CorsConfig {
                allowed_origins: env::var("ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| "http://localhost:3000".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
            password_reset: PasswordResetConfig {
                expiration: env::var("PASSWORD_RESET_EXPIRATION")
                    .unwrap_or_else(|_| "3600".to_string()) // 1 hour in seconds
                    .parse()
                    .expect("PASSWORD_RESET_EXPIRATION must be a number"),
            },
            payment: PaymentConfig {
                mtn_api_key: env::var("MTN_API_KEY").unwrap_or_default(),
                mtn_api_secret: env::var("MTN_API_SECRET").unwrap_or_default(),
                orange_api_key: env::var("ORANGE_API_KEY").unwrap_or_default(),
                orange_api_secret: env::var("ORANGE_API_SECRET").unwrap_or_default(),
            },
        }
    })
}
