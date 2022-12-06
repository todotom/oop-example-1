#[cfg(test)]
mod test_user_controller {
    use crate::settings::{USER_DEFAULT_SIGN_IN_COUNT, USER_DEFAULT_STATUS};
    use crate::user::user::User;
    use crate::user::user_controller::UserController;
    use crate::user::user_repository::get_user_result::GetUserResult;
    use crate::user::user_repository::user_mock_repository::UserMockRepository;
    use crate::user::user_repository::user_repository::UserRepository;

    #[test]
    fn test_found_user() {
        let mocked_email: &str = "mocked@email.com";
        let mocked_username: &str = "MOCK";
        let found_user: bool = true;
        let user_repository: Box<dyn UserRepository> = Box::new(UserMockRepository::new(
            found_user,
            String::from(mocked_email),
            String::from(mocked_username),
        ));
        let user_controller: UserController = UserController::create(user_repository);
        let _user_result: GetUserResult = user_controller.get();
        assert!(matches!(
            GetUserResult::User(User::new(
                USER_DEFAULT_STATUS,
                String::from(mocked_email),
                String::from(mocked_username),
                USER_DEFAULT_SIGN_IN_COUNT,
            )),
            _user_result
        ));
    }

    #[test]
    fn test_not_found_user() {
        let found_user: bool = true;
        let user_repository: Box<dyn UserRepository> = Box::new(UserMockRepository::new(
            found_user,
            String::from(""),
            String::from(""),
        ));
        let user_controller: UserController = UserController::create(user_repository);
        let _user_result: GetUserResult = user_controller.get();
        assert!(matches!(GetUserResult::UserNotFound, _user_result));
    }
}
