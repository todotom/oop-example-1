use crate::user::{
    new_user::NewUser,
    user::User,
    user_repository::{get_user_result::GetUserResult, user_repository::UserRepository},
};

pub struct UserMockRepository {
    found_user: bool,
}

impl UserMockRepository {
    pub fn new(found_user: bool) -> Self {
        return Self { found_user };
    }
}

impl UserRepository for UserMockRepository {
    fn get(&self) -> GetUserResult {
        if self.found_user {
            return GetUserResult::User(User::create_from_new_user(NewUser {
                email: String::from("mocked@email.com"),
                username: String::from("MOCK"),
            }));
        }
        return GetUserResult::UserNotExist;
    }
}
