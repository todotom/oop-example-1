use crate::user::user_repository::{
    get_user_result::GetUserResult, user_repository::UserRepository,
};

pub struct UserController {
    user_repository: Box<dyn UserRepository>,
}

impl UserController {
    pub fn create(user_repository: Box<dyn UserRepository>) -> Self {
        return UserController { user_repository };
    }

    pub fn get(&self) -> GetUserResult {
        return self.user_repository.get();
    }
}
