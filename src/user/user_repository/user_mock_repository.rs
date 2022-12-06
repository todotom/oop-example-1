use crate::user::{
    new_user::NewUser,
    user::User,
    user_repository::{get_user_result::GetUserResult, user_repository::UserRepository},
};

pub struct UserMockRepository {
    found_user: bool,
    email: String,
    username: String,
}

impl UserMockRepository {
    pub fn new(found_user: bool, email: String, username: String) -> Self {
        return Self {
            found_user,
            email,
            username,
        };
    }
}

impl UserRepository for UserMockRepository {
    fn get(&self) -> GetUserResult {
        if self.found_user {
            return GetUserResult::User(User::create_from_new_user(NewUser {
                email: self.email.to_owned(),
                username: self.username.to_owned(),
            }));
        }
        return GetUserResult::UserNotFound;
    }
}
