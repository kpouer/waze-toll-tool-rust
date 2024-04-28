use crate::hash;

pub(crate) mod user_repository;

#[derive(Clone)]
pub(crate) struct User {
    pub(crate) username: String,
    /**
     * The password is salted using BCrypt
     */
    pub(crate) password_hash: String,
    pub(crate) teams: Vec<String>,
    pub(crate) admin: bool
}

impl User {
    pub(crate) fn is_valid(&self, password: &str) -> bool {
        hash::check(&self.password_hash, password)
    }
}
