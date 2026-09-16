use std::{
    f64::consts::E,
    fs::File,
    io::{self, Read},
};

fn main() {
    let v = vec![1, 2, 3];
    assert_eq!(Some(3), queue_ticket(&v, 2));
    assert_eq!(None, queue_ticket(&v, 100));

    if let Err(err) = File::open("non_existsing_file.txt") {
        println!("Open non existsing file error: {}", err)
    };
    if let Ok(mut file) = File::open("hello.txt") {
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .unwrap_or_else(|e| panic!("failed to read hello.txt: {}", e));

        println!("Hello.txt content is: {}", file_content);
    }

    assert_eq!(
        "Rustler",
        read_username_from_file().unwrap_or(String::new())
    );

    assert_eq!(Err(String::from("invalid age")), parse_visitor_age("dsdaw"));
    assert_eq!(Err(String::from("invalid age")), parse_visitor_age("-1"));
    assert_eq!(Err(String::from("too old")), parse_visitor_age("121"));
    assert_eq!(Ok(120), parse_visitor_age("120"));

    assert_eq!(Err(String::from("division by zero")), share_boxes(10, 0));
    assert_eq!(Ok(2), share_boxes(4, 2));

    assert_eq!(Err(LogError::Empty), parse_minutes(""));
    assert_eq!(Err(LogError::NotNumber), parse_minutes("dsad"));
    assert_eq!(Ok(32), parse_minutes("32"));
    assert_eq!(Ok(32), parse_minutes(" 32"));
}

fn queue_ticket(tickets: &[i32], index: usize) -> Option<i32> {
    tickets.get(index).copied()
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;

    username = username.trim().to_string();
    Ok(username)
}

fn parse_visitor_age(input: &str) -> Result<i32, String> {
    let age: i32 = input.parse().map_err(|_| String::from("invalid age"))?;
    match age {
        0..=120 => Ok(age),
        age if age < 0 => Err(String::from("invalid age")),
        _ => Err(String::from("too old")),
    }
}

fn share_boxes(items: i32, boxes: i32) -> Result<i32, String> {
    if boxes == 0 {
        return Err(String::from("division by zero"));
    }

    Ok(items / boxes)
}

#[derive(Debug, PartialEq)]
enum LogError {
    Empty,
    NotNumber,
}

fn parse_minutes(input: &str) -> Result<u32, LogError> {
    let input = input.trim();
    match input {
        x if x.is_empty() => Err(LogError::Empty),
        _ => input.parse().map_err(|_| LogError::NotNumber),
    }
}
