pub(crate) fn salt(password: &str) -> String {
    bcrypt::hash(password, 4).unwrap()
}

pub(crate) fn check(expected_password_hash: &str, password: &str) -> bool {
    bcrypt::verify(password, &expected_password_hash).unwrap()
}