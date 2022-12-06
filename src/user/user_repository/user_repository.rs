use crate::user::user_repository::get_user_result::GetUserResult;

pub trait UserRepository {
    fn get(&self) -> GetUserResult;
}
