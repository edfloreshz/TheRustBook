use crate::UserState::Inactive;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
enum UserState {
    Active,
    Inactive,
    Blocked,
    Deleted
}
#[derive(Debug, Clone)]
struct User {
    name: String,
    username: String,
    password: String,
    state: UserState
}

impl User {
    fn new(name: String, username: String, password: String) -> User {
        User {name, username, password, state: UserState::Active }
    }
    fn from(user: &User) -> User {
        User {..*user }
    }
}

fn main() {
    let user = User::new(
        String::from("Eduardo Flores"),
        String::from("edfloreshz"),
        String::from("password")
    );

    let mut duplicate = User::from(user.borrow());
    if let UserState::Active = duplicate.state {
        duplicate.state = UserState::Inactive
    }

    println!("User password status is: {:?} and duplicate's status is: {:?}", user.state, duplicate.state);
}
