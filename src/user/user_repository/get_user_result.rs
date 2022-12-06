use crate::user::user::User;

pub enum GetUserResult {
    User(User),
    UserNotExist,
}
