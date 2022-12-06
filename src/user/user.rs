use super::new_user::NewUser;

pub struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}

impl User {
    pub fn create_from_new_user(new_user: NewUser) -> Self {
        return Self {
            email: new_user.email,
            username: new_user.username,
            active: false,
            sign_in_count: 0,
        };
    }

    pub fn get_username(&self) -> &String {
        return &self.username;
    }

    pub fn get_email(&self) -> &String {
        return &self.email;
    }
}
