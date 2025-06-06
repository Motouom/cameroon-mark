// Local modules
mod config;

// Import external crates
use std::sync::Arc;
use std::net::SocketAddr;
use aws_sdk_s3::Client as S3Client;
use aws_config::BehaviorVersion;
use aws_sdk_s3::config::{Region, Credentials};
use aws_config::meta::region::RegionProviderChain;

// Import the AppState and routes from lib.rs
use cameroon_mark_backend::{AppState, routes};

use axum::{
    routing::get,
    Router,
    http::{HeaderValue, Method, HeaderName},
    extract::Extension,
};
use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = config::get_config();

    // Set up logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Cameroon Market API server");

    // Set up database connection with explicit pool settings
    let mut opt = ConnectOptions::new(config.database.url.clone());

    // Configure connection pool settings
    opt.max_connections(config.database.max_connections)
       .min_connections(1)
       .connect_timeout(Duration::from_secs(10))
       .acquire_timeout(Duration::from_secs(10))
       .idle_timeout(Duration::from_secs(300))
       .max_lifetime(Duration::from_secs(1800))
       .sqlx_logging(true);

    let db = Database::connect(opt).await?;

    tracing::info!("Connected to database");

    // Set up S3 client for MinIO
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));
    let credentials = Credentials::new(
        &config.minio.access_key,
        &config.minio.secret_key,
        None,
        None,
        "minio"
    );

    let s3_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .endpoint_url(&config.minio.endpoint)
        .credentials_provider(credentials)
        .load()
        .await;

    let s3_client = S3Client::new(&s3_config);

    tracing::info!("Connected to MinIO");

    // Create a CORS layer
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            HeaderName::from_static("authorization"),
            HeaderName::from_static("content-type"),
            HeaderName::from_static("x-requested-with"),
            HeaderName::from_static("accept"),
            HeaderName::from_static("origin"),
            HeaderName::from_static("x-csrftoken"),
            HeaderName::from_static("x-xsrf-token"),
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600))
        .expose_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("content-length"),
            HeaderName::from_static("authorization"),
        ])
        .allow_origin("http://localhost:8083".parse::<HeaderValue>().unwrap());

    // Convert local config to the lib Config type
    let lib_config = cameroon_mark_backend::Config {
        database: cameroon_mark_backend::config::DatabaseConfig {
            url: config.database.url.clone(),
            max_connections: config.database.max_connections,
        },
        server: cameroon_mark_backend::config::ServerConfig {
            host: config.server.host.clone(),
            port: config.server.port,
        },
        jwt: cameroon_mark_backend::config::JwtConfig {
            secret: config.jwt.secret.clone(),
            expiration: config.jwt.expiration,
        },
        minio: cameroon_mark_backend::config::MinioConfig {
            endpoint: config.minio.endpoint.clone(),
            access_key: config.minio.access_key.clone(),
            secret_key: config.minio.secret_key.clone(),
            bucket: config.minio.bucket.clone(),
            region: config.minio.region.clone(),
            public_url: config.minio.public_url.clone(),
        },
        cors: cameroon_mark_backend::config::CorsConfig {
            allowed_origins: config.cors.allowed_origins.clone(),
            allowed_methods: config.cors.allowed_methods.clone(),
            allowed_headers: config.cors.allowed_headers.clone(),
            max_age_secs: config.cors.max_age_secs,
        },
        password_reset: cameroon_mark_backend::config::PasswordResetConfig {
            expiration: config.password_reset.expiration,
        },
        payment: cameroon_mark_backend::config::PaymentConfig {
            mtn_api_key: config.payment.mtn_api_key.clone(),
            mtn_api_secret: config.payment.mtn_api_secret.clone(),
            orange_api_key: config.payment.orange_api_key.clone(),
            orange_api_secret: config.payment.orange_api_secret.clone(),
        },
    };

    // Set up application state
    let app_state = Arc::new(AppState {
        db: Arc::new(db),
        s3_client: Arc::new(s3_client),
        config: lib_config,
    });

    // Set up API routes
    let app = Router::new()
        .route("/", get(health_check))
        .nest("/api", api_routes(app_state.clone()))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(app_state);

    // Start the server
    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::IpAddr>()?,
        config.server.port,
    ));

    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// Health check endpoint
async fn health_check() -> &'static str {
    "Cameroon Market API is running"
}

// API routes
fn api_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", routes::auth::routes())
        .nest("/users", routes::user::routes())
        .nest("/products", routes::product::routes())
        .nest("/categories", routes::category::routes())
        .nest("/upload", routes::upload::routes())
        .nest("/cart", routes::cart::routes())
        .nest("/orders", routes::order::routes())
        .nest("/saved-items", routes::saved_item::routes())
        .nest("/admin", routes::admin::routes())
        .nest("/analytics", routes::analytics::routes())
        .nest("/marketing", routes::marketing::routes(app_state.clone()))
}
