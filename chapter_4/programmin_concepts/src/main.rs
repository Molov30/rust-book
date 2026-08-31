use std::cmp::Ordering;

fn main() {
    variables_and_mutable();
    data_types();
    another_function(123);
    println!("{} square is: {}", 9, square(9));
    println!("{} minutes is {} seconds", 3, minute_to_seconds(3));
    println!("{} is even? {}", 6, is_even(6));
    control_flow(25);
    println!("Max of (32, 12) is: {}", max_of_two(32, 12));
    println!("Sum number to 3: {}", sum_to(3));
    println!("12 ceslius is {} farenheits", convert_temperature(12));
    println!("10 fibonacci is {}", fibonacci(10));
    println!("repka 30 lines:\n{}", repka_chain(30));
}

fn variables_and_mutable() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;  ERROR: cannot assign twice ummutable variable 'x'
    println!("The value of x is: {}", x);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // THREE_HOURS_IN_SECONDS = 3; ERROR: invalid left-side in assigment, constants is ummutable
    println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

    let x = 5;
    let x = 6; // shadowed x = 5

    {
        let x = 7; // shadowed x = 6
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "     "; // str
    let spaces = spaces.len(); // uszie
    println!("spaces len is: {}", spaces);
}

fn data_types() {
    let _: i8 = -1;
    let _: u8 = 1;

    let _: i16 = -1;
    let _: u16 = 1;

    let _: i32 = -1;
    let _: u32 = 1;

    let _: i64 = -1;
    let _: u64 = 1;

    let _: i128 = -1;
    let _: u128 = 1;

    // platform specific
    let _: isize = -1;
    let _: usize = 1;

    let _: f32 = -3.14;
    let _: f64 = 3.14;

    let sum = 1 + 3;
    let difference = 3 - 1;
    let multiplication = 3 * 2;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1
    let remainder = 43 % 5;

    let _: bool = true;
    let _: bool = false;

    let _: char = 'z';
    let _ = '😻';

    let tuple: (i32, char, f64) = (3, '😻', 3.14);
    let (x, y, z) = tuple; // Destructuring
    println!(
        "Tuple elements: firts is {}, second is {}, third is {}",
        tuple.0, tuple.1, tuple.2
    );
    let _ = (); // empty tuple aka Unit

    let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!(
        "Array elemetns: first is {}, second is {}, third is {}",
        array[0], array[1], array[2]
    );
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x)
}

fn square(x: i32) -> i32 {
    x * x
}

fn minute_to_seconds(minutes: u32) -> u32 {
    minutes * 60
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn control_flow(x: i32) {
    let txt = if is_even(x) {
        format!("{} is even", x)
    } else {
        format!("{} not even", x)
    };
    println!("{}", txt);
}

fn max_of_two(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    b
}

fn sum_to(n: u64) -> u64 {
    let mut sum = 0;
    for num in 1..=n {
        sum += num;
    }
    sum
}

fn convert_temperature(celsius: i32) -> i32 {
    celsius * 9 / 5 + 32
}

fn fibonacci(n: u32) -> u64 {
    if n < 2 {
        return n as u64;
    }

    let mut prev = 0;
    let mut fib = 1;

    for _ in 1..n {
        let temp = fib;
        fib += prev;
        prev = temp;
    }
    fib
}

fn repka_chain(n: usize) -> String {
    if n == 0 {
        return String::from("");
    }

    let lines: Vec<&str> = "дедка за репку
                бабка за дедку
                внучка за бабку
                Жучка за внучку
                кошка за Жучку
                мышка за кошку"
        .split("\n")
        .map(|x| x.trim())
        .collect();

    let n = match n.cmp(&lines.len()) {
        Ordering::Less => n,
        _ => lines.len(),
    };

    lines[..n].join("\n")
}
