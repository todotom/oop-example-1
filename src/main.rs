// use user::user_controller::UserController;
// use user::user_repository::get_user_result::GetUserResult;
// use user::user_repository::user_mock_repository::UserMockRepository;
// use user::user_repository::user_repository::UserRepository;

use user::{
    user_controller::UserController,
    user_repository::{
        get_user_result::GetUserResult, user_mock_repository::UserMockRepository,
        user_repository::UserRepository,
    },
};

mod user;

fn main() {
    let found_user: bool = true;
    let user_repository: Box<dyn UserRepository> = Box::new(UserMockRepository::new(found_user));
    let user_controller: UserController = UserController::create(user_repository);
    let user_result: GetUserResult = user_controller.get();
    match user_result {
        GetUserResult::User(user) => println!(
            "username: {}\nemail: {}",
            user.get_username(),
            user.get_email()
        ),
        GetUserResult::UserNotExist => println!("user not exist"),
    }
}

// fn main() {
//     let username: String = String::from("todotom");
//     let email: String = String::from("tomasdarioam@protonmail.com");
//     let user: User = User::create_from_new_user(NewUser {
//         email: email,
//         username: username,
//     });
//     println!("username: {}", user.get_username());
//     println!("email: {}", user.get_email());
// }
