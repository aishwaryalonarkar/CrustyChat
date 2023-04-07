use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HashData {
    pub master: String,
    pub slave_replica: String,
    pub slave_backup: String,
    pub master_hash: String,
    pub slave_backup_hash: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLoginData {
    pub password: String,
    pub username: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserToken {
    pub username : String,
    pub is_active_user : bool,
    pub is_valid_user : bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsernameAdd {
    pub user: String,
    pub username: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserData {
    pub name: String,
    pub phone: String,
    pub password: String,
    pub email: String,
    pub hash: String,
    pub username: String,
    pub chats: String
}
impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " password: {}, email: {}, username: {}, chats: {}", self.password, self.email,self.username,self.chats)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    status: u16,
    message: String,
}


impl Response {
    pub fn new() -> Response {
        Response {
            status: 0,
            message: String::new(),
        }
    }

    pub fn set_status(&mut self, status: u16) {
        self.status = status;
    }

    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "status: {}, message: {}", self.status, self.message)
    }
}
