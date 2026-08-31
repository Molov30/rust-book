enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn is_quit(&self) -> bool {
        match self {
            Message::Quit => true,
            _ => false,
        }
    }
}

enum BakeryOrder {
    Bread(u32),
    Cake { inscription: String, candles: u8 },
    Coffee,
}

fn order_bread(val: u32) -> BakeryOrder {
    BakeryOrder::Bread(val)
}

fn order_cake(inscription: String, candles: u8) -> BakeryOrder {
    BakeryOrder::Cake {
        inscription,
        candles,
    }
}

fn order_coffee() -> BakeryOrder {
    BakeryOrder::Coffee
}

enum RemoteCommand {
    Sleep,
    Temperature(i8),
    Radio { station: String, volume: u8 },
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn wait_seconds(light: &TrafficLight) -> u8 {
    match light {
        TrafficLight::Red => 60,
        TrafficLight::Yellow => 5,
        TrafficLight::Green => 0,
    }
}

enum Delivery {
    New,
    InTransit(u8),
    Delivered,
}

fn remaining_days(delivery: &Delivery) -> u8 {
    match delivery {
        Delivery::New => 7,
        Delivery::InTransit(days) => *days,
        Delivery::Delivered => 0,
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn describe(&self) -> &str {
        match self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        }
    }
}

fn describe_number(val: Option<i32>) -> String {
    if let Some(num) = val {
        return format!("number: {}", num);
    }

    format!("none")
}

fn access_label(val: Option<u8>) -> String {
    let Some(num) = val else {
        return format!("missing");
    };

    match num {
        0 => "guest".to_string(),
        1..=3 => "user".to_string(),
        4..=10 => "admin".to_string(),
        _ => "invalid".to_string(),
    }
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    let msg = Message::Write(String::from("Hi"));
    assert!(!msg.is_quit());

    match get_element(-1) {
        Some(value) => assert_eq!(3, value),
        None => println!("Value is none"),
    }

    assert_eq!(25, value_in_cents(&Coin::Quarter(UsState::Alabama)));
}

fn get_element(idx: isize) -> Option<i64> {
    if idx < 0 {
        return None;
    }

    Some(idx as i64)
}
