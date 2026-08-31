fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("some@example.com"),
        sign_in_count: 43,
    };

    user1.email = String::from("another@examile.com");

    _ = build_user("someuser", "somemail@example.com");

    let line = make_line("velikii sup".to_string(), 10, 32);
    assert_eq!(320, line.total);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // assert_eq!(user1.username, user2.username); ERROR: user1.username value moved to user2.username
    assert_eq!(user1.active, user2.active);
    assert_eq!(user1.sign_in_count, user2.sign_in_count);

    let _red = Color(255, 0, 0);

    let my_box = BoxSize(3, 3, 3);
    assert_eq!(27, volume_cm3(my_box));
}

fn build_user(username: &str, email: &str) -> User {
    User {
        active: true,
        username: username.to_string(),
        email: email.to_string(),
        sign_in_count: 1,
    }
}

fn make_line(item: String, unit_price: u32, quantity: u32) -> ReceiptLine {
    ReceiptLine {
        item,
        unit_price,
        quantity,
        total: unit_price * quantity,
    }
}

fn volume_cm3(size: BoxSize) -> u32 {
    size.0 * size.1 * size.2
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct ReceiptLine {
    item: String,
    unit_price: u32,
    quantity: u32,
    total: u32,
}

struct Color(i32, i32, i32);

struct BoxSize(u32, u32, u32);
