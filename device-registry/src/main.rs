mod bcrypt;
mod devices;
mod health;
mod users;

use axum::{
    routing::{get, post},
    Router,
};
use devices::create::create_device;
use health::check::health_check;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use users::{create::create_user, read::read_user};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Bootstrapping device registry...");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/users", post(create_user))
        .route("/users/:id", get(read_user))
        .route("/devices", post(create_device))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

    tracing::info!(
        "Listening on port {}...",
        listener.local_addr().unwrap().port()
    );

    axum::serve(listener, app).await.unwrap();
}
