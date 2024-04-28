use log::{info, warn};
use crate::hash;
use crate::user::User;

#[derive(Clone)]
pub(crate) struct UserRepository {
    pub(crate) users: Vec<User>
}

impl Default for UserRepository {
    fn default() -> Self {
        UserRepository {
            users: vec![
                User {
                    username: "admin".to_string(),
                    password_hash: hash::salt(&"admin".to_string()),
                    teams: vec!["admin".to_string()],
                    admin: true
                },
                User {
                    username: "user".to_string(),
                    password_hash: hash::salt(&"user".to_string()),
                    teams: vec!["tc_france".to_string()],
                    admin: false
                },
                User {
                    username: "user2".to_string(),
                    password_hash: hash::salt(&"user".to_string()),
                    teams: vec!["tc_france".to_string()],
                    admin: false
                }
            ]
        }
    }
}

impl UserRepository {
    pub(crate) fn is_user_valid(&self, username: &str, password: Option<String>) -> bool {
        if password.is_none() {
            warn!("Password is None");
            return false;
        }
        let password = &password.unwrap();
        return match self.find_user(username) {
            None => {
                warn!("User not found {}", username);
                false
            }
            Some(user) => {
                info!("User found {}", username);
                user.is_valid(password)
            }
        }
    }

    pub(crate) fn list_teams(&self) -> Vec<String> {
        let mut teams = Vec::new();
        for user in &self.users {
            for team in &user.teams {
                if !teams.contains(team) {
                    teams.push(team.clone());
                }
            }
        }
        teams
    }

    fn find_user(&self, username: &str) -> Option<&User> {
        self.users
            .iter()
            .find(|user| user.username == *username)
    }
}