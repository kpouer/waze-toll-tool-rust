use std::fs;
use std::path::Path;

use log::{info, warn};
use sqlx::{Error, Executor, Pool, Row, Sqlite, SqlitePool};
use sqlx::sqlite::SqliteRow;

use crate::{hash, info};
use crate::model::user::User;
use crate::router::admin::list_teams;

#[derive(Clone)]
pub(crate) struct UserRepository {
    pool: Pool<Sqlite>,
}

impl UserRepository {
    pub(crate) async fn new() -> Result<Self, sqlx::Error> {
        let (pool, should_init_db) = get_database_pool().await;
        let repository = UserRepository {
            // users: vec![
            //     User {
            //         username: "admin".to_string(),
            //         password_hash: hash::salt(&"admin".to_string()),
            //         teams: vec!["admin".to_string()],
            //         admin: true,
            //     },
            //     User {
            //         username: "user".to_string(),
            //         password_hash: hash::salt(&"user".to_string()),
            //         teams: vec!["tc_france".to_string()],
            //         admin: false,
            //     },
            //     User {
            //         username: "user2".to_string(),
            //         password_hash: hash::salt(&"user".to_string()),
            //         teams: vec!["tc_france".to_string()],
            //         admin: false,
            //     },
            // ],
            pool
        };
        
        if should_init_db {
            repository.init_db().await;
        }
        Ok(repository)
    }

    async fn init_db(&self) {
        let query = "
        CREATE TABLE team (
            name TEXT PRIMARY KEY
        );
        CREATE TABLE user (
            username TEXT PRIMARY KEY,
            password_hash TEXT NOT NULL,
            admin BOOLEAN NOT NULL DEFAULT FALSE
        );
    ";
        self.pool.execute(query).await.unwrap();
        info!("Database initialized");

        let admin_user = User {
            username: "admin".to_string(),
            password_hash: hash::salt(&"admin".to_string()),
            teams: vec!["admin".to_string()],
            admin: true,
        };
        self.insert_user(&admin_user).await;
    }

    pub(crate) async fn insert_user(&self, user: &User) {
        let query = "INSERT INTO user (username, password_hash, admin) VALUES (?, ?, ?)";
        sqlx::query(query)
            .bind(&user.username)
            .bind(&user.password_hash)
            .bind(&user.admin)
            .execute(&self.pool)
            .await
            .unwrap();
        // todo : check result
        for team in &user.teams {
            self.link_user_team(&user.username, team).await;
        }
    }

    pub(crate) async fn is_valid_for_team(&self, username: &str, password: Option<String>, team: &String) -> bool {
        self.is_user_valid(username, password).await && self.has_team(username, team).await
    }

    pub(crate) async fn is_user_valid(&self, username: &str, password: Option<String>) -> bool {
        if password.is_none() {
            warn!("Password is None");
            return false;
        }
        let password = &password.unwrap();
        return match self.find_user(username).await {
            None => {
                warn!("User not found {}", username);
                false
            }
            Some(user) => {
                info!("User found {}", username);
                user.is_valid(password)
            }
        };
    }

    pub(crate) async fn has_team(&self, username: &str, team: &String) -> bool {
        match self.find_user(username).await {
            None => {
                warn!("User not found {}", username);
                false
            }
            Some(user) => {
                info!("User found {}", username);
                user.teams.contains(&team.to_string())
            }
        }
    }

    pub(crate) async fn list_teams(&self) -> Vec<String> {
        info!("list_teams");
        let teams = sqlx::query("SELECT name FROM team ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .iter()
            .map(|row| row.get(0))
            .collect();
        teams
    }
    
    pub(crate) async fn list_users(&self) -> Vec<String> {
        info!("list_users");
        let users = sqlx::query("SELECT username FROM user ORDER BY username")
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .iter()
            .map(|row| row.get(0))
            .collect();
        users
    }

    async fn find_user(&self, username: &str) -> Option<User> {
        let query = "SELECT username, password_hash, admin FROM user WHERE username = ?";
        let row = sqlx::query(query)
            .bind(username)
            .fetch_one(&self.pool)
            .await
            .ok();
        match row {
            None => None,
            Some(row) => {
                let teams = self.find_user_teams(username).await;
                Some(User {
                    username: row.get(0),
                    password_hash: row.get(1),
                    teams,
                    admin: row.get(2),
                })
            }
        }
    }

    async fn find_user_teams(&self, username: &str) -> Vec<String> {
        let query = "SELECT DISTINCT team FROM user_team WHERE username = ?";
        let mut rows = sqlx::query(query)
            .bind(username)
            .fetch_all(&self.pool)
            .await;
        match rows {
            Ok(rows) => return rows.iter().map(|row| row.get(0)).collect(),
            Err(err) => warn!("Error fetching teams for user {}: {}", username, err)
        }

        vec![]
    }
    async fn link_user_team(&self, username: &str, team: &str) {
        info!("insert_user_team {} {}", username, team);
        self.insert_team(team).await;
        
    }

    async fn insert_team(&self, team: &str) {
        info!("insert_team {}", team);
        let query = "INSERT INTO team (name) VALUES (?)";
        sqlx::query(query)
            .bind(team)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}

async fn get_database_pool() -> (Pool<Sqlite>, bool) {
    let db_path = "database/users";
    let should_init_db = !Path::new(db_path).exists();
    if should_init_db {
        fs::create_dir_all("database").unwrap();
        fs::File::create(db_path).expect("Unable to create database file");
    }
    let pool = SqlitePool::connect(&format!("sqlite:{}", db_path)).await?;
    (pool, should_init_db)
}


