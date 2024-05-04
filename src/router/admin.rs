use std::collections::HashSet;

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::routing::get;
use log::info;

use crate::{hash, RoadworkServerData};

pub(crate) fn admin_routes() -> Router<RoadworkServerData> {
    Router::new()
        .route("/teams", get(list_teams))
        .route("/users", get(list_users))
        .route("/check/:bcrypted/:password", get(check))
        .route("/salt/:password", get(salt))
}

pub(crate) async fn list_teams(State(state): State<RoadworkServerData>) -> Json<Vec<String>> {
    info!("list_teams");
    let teams = state.user_repository.list_teams().await;
    info!("list_teams -> {:?}", teams);
    Json(teams)
}

pub(crate) async fn list_users(State(state): State<RoadworkServerData>) -> Json<Vec<String>> {
    info!("list_users");
    let user_names = state.user_repository.list_users().await;
    info!("list_users -> {:?}", user_names);
    Json(user_names)
}

pub(crate) async fn check(Path((bcrypted, password)): Path<(String, String)>) -> &'static str {
    info!("check XXXXXXX");
    let result = hash::check(bcrypted.as_str(), password.as_str());
    if result {
        "Password is correct"
    } else {
        "Password is incorrect"
    }
}

pub(crate) async fn salt(Path(password): Path<String>) -> String {
    info!("Salt XXXXXXX");
    let salted_password = hash::salt(&password);
    let response = format!("Bcrypt {} -> {}", password, salted_password);
    info!("Salt XXXXXXX -> {}", salted_password);
    response
}
