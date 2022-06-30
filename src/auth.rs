use bcrypt::{hash, verify, DEFAULT_COST};

pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_owned(),
            password: hash(password, DEFAULT_COST).unwrap(),
        }
    }

    pub const fn from_db(username: String, password: String) -> Self {
        Self { username, password }
    }
    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password).unwrap()
    }
}
