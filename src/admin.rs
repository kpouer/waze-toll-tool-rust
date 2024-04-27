use std::collections::HashSet;

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::routing::get;
// Module: admin
use log::info;

use crate::{hash, RoadworkServerData};

pub(crate) async fn list_teams(State(state): State<RoadworkServerData>) -> Vec<String> {
    let user_repository = &state.user_repository;
    let users = &user_repository.users;
    let mut teams = HashSet::new();
    users
        .iter()
        .flat_map(|user| user.teams.clone())
        .for_each(|team| {
            teams.insert(team);
        });
    teams.into_iter().collect()
}

pub(crate) async fn list_users(State(state): State<RoadworkServerData>) -> Vec<String> {
    let user_repository = &state.user_repository;
    let users = &user_repository.users;
    let user_names = users
        .iter()
        .map(|user| user.username.clone())
        .collect::<Vec<String>>();
    user_names
}

pub(crate) async fn check(Path((bcrypted, password)): Path<(String, String)>) -> &'static str {
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

pub(crate) fn admin_routes() -> Router {
    Router::new()
        .route("/admin/teams", get(list_teams))
        .route("/admin/users", get(list_users))
        .route("/admin/check/:bcrypted/:password", get(check))
        .route("/admin/salt/:password", get(salt))
}