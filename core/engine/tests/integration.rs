// Integration tests
use engine::engine::{
    engine,
    User
};

#[test]
fn add_two() {
    let result: i8 = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn test_main() {
    let user1 = engine();
    let user2 = User {
        name: String::from("John"),
        email: String::from("gmail.com"),
        password: String::from("1234"),
        wallet_address: String::from("0x1234")
    };
    assert_eq!(user1, user2);
}