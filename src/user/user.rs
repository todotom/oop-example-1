use crate::settings::{USER_DEFAULT_SIGN_IN_COUNT, USER_DEFAULT_STATUS};

use crate::user::new_user::NewUser;

pub struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}

impl User {
    pub fn new(active: bool, email: String, username: String, sign_in_count: u64) -> Self {
        Self {
            active,
            email,
            username,
            sign_in_count,
        }
    }

    pub fn create_from_new_user(new_user: NewUser) -> Self {
        return Self {
            email: new_user.email.to_string(),
            username: new_user.username,
            active: USER_DEFAULT_STATUS,
            sign_in_count: USER_DEFAULT_SIGN_IN_COUNT,
        };
    }

    pub fn get_username(&self) -> &String {
        return &self.username;
    }

    pub fn get_email(&self) -> &String {
        return &self.email;
    }
}
