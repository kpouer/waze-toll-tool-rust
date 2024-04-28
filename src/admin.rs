use std::collections::HashSet;

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::routing::get;
use log::info;

use crate::{hash, RoadworkServerData};

pub(crate) async fn list_teams(State(state): State<RoadworkServerData>) -> Json<Vec<String>> {
    info!("list_teams");
    let mut teams = HashSet::new();
    state.user_repository.users
        .iter()
        .flat_map(|user| user.teams.clone())
        .for_each(|team| {
            teams.insert(team);
        });
    let result = teams.into_iter().collect();
    Json(result)
}

pub(crate) async fn list_users(State(state): State<RoadworkServerData>) -> Json<Vec<String>> {
    info!("list_users");
    let user_names = state.user_repository.users
        .iter()
        .map(|user| user.username.clone())
        .collect::<Vec<String>>();
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

pub(crate) fn admin_routes() -> Router<RoadworkServerData> {
    Router::new()
        .route("/admin/teams", get(list_teams))
        .route("/admin/users", get(list_users))
        .route("/admin/check/:bcrypted/:password", get(check))
        .route("/admin/salt/:password", get(salt))
}