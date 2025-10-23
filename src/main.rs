use axum::{routing::get, Router};
use std::sync::{Arc, RwLock};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod game;
mod routes;

use routes::game::SharedGameState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_emperor=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create shared game state
    let game_state: SharedGameState = Arc::new(RwLock::new(None));

    // Build our application with routes
    let app = Router::new()
        // Serve static files
        .nest_service("/static", ServeDir::new("static"))
        // Menu routes (no state needed)
        .merge(routes::menu_routes())
        // Game routes (need state)
        .merge(routes::game_routes())
        // Add state
        .with_state(game_state)
        // Add middleware
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // Run the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("🏰 Dark Emperor server starting on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
