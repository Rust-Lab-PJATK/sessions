use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Default)]
pub struct User {
    pub login: String,
    pub password: String
}

impl User {
    pub fn new(login: String, password: String) -> Self {
        Self { login, password }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User -> login: {}, password: {}", self.login, self.password)
    }
}

// impl Default for User {
//     fn default() -> Self {
//         Self { login: "xd".to_string(), password: "xd".to_string() }
//     }
// }