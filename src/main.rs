use axum::extract::State;
use axum::Router;
use axum::routing::get;
use axum_auth::AuthBasic;
use log::info;
use crate::user::user_repository::UserRepository;

mod admin;
mod hash;
mod user;
mod security;

#[derive(Clone)]
pub(crate) struct RoadworkServerData {
    user_repository: UserRepository
}

async fn info() -> &'static str {
   "Rust server is running"
}

async fn info_secure(AuthBasic((id, password)): AuthBasic, State(state): State<RoadworkServerData>) -> &'static str {
    info!("User: {}", id);
    if state.user_repository.is_user_valid(id.as_str(), password) {
        info!("User is valid");
        "User is valid"
    } else {
        info!("User is invalid");
        "User is invalid"
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Starting Roadwork server");
    let roadwork_server_data = RoadworkServerData {
        user_repository: UserRepository::default()
    };
    let app = Router::new()
        .route("/info", get(info))
        .route("/info_secure", get(info_secure))
        .with_state(roadwork_server_data)
        ;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("Listen on 0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
