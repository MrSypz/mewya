use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    // // dotenv
    // dotenvy::dotenv().ok();

    // // logging system
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::INFO)
    //     .init();

    // // database connection
    // let db_pool = match create_pool().await {
    //     Ok(pool) => pool,
    //     Err(e) => {
    //         tracing::error!("Failed to connect to database: {}", e);
    //         std::process::exit(1);
    //     }
    // };
    // tracing::info!("✅ Successfully connected to database");

    // // Cors
    // let cors = CorsLayer::new()
    //     .allow_origin([
    //         "http://127.0.0.1:port".parse().unwrap(),
    //     ])
    //     .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    //     .allow_headers([
    //         HeaderName::from_static("content-type"),
    //         HeaderName::from_static("authorization"),
    //     ])
    //     .allow_credentials(true);

    // // web socket
    // let addr = SocketAddr::from(([0, 0, 0, 0], 9696));
    // tracing::info!("🚀 Server running at http://{}", addr);
    // let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // let app = Router::new()
    //     .route("/", get(hello_world))
    //     .with_state(db_pool)
    //     .layer(cors);

    // axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();

    // Default settings
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Json<Value> {
    Json(json!({
        "message": "Hello, World!",
        "status": "ok"
    }))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
