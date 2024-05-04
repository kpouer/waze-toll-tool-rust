use axum::Router;
use axum::routing::get;
use log::info;

use crate::router::admin::admin_routes;
use crate::user_repository::UserRepository;

mod hash;
mod router;
mod model;
mod service;
mod user_repository;

#[derive(Clone)]
pub(crate) struct RoadworkServerData {
    user_repository: UserRepository,
}

async fn info() -> &'static str {
    "Rust server is running"
}

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Starting Roadwork server");
    let roadwork_server_data = RoadworkServerData {
        user_repository: UserRepository::new().await.unwrap(),
    };
    let app = Router::new()
        .nest("/admin", admin_routes())
        .with_state(roadwork_server_data)
        ;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("Listen on 0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
