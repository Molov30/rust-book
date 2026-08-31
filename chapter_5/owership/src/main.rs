fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("{}", s1);

    let s2 = s1;
    // println!("{}", s1);  Error: s1 value moved to s2

    let s3 = take_and_return(s2);
    // println!("{}", s2); Error: s2 value to moved to take_and_return fn

    println!("s3 len: {}", calculate_length(&s3));
    println!("{}", s3);
}

fn take_and_return(s: String) -> String {
    s
}

fn choose_label(primary: String, reserve: String, use_primary: bool) -> String {
    if use_primary {
        return primary;
    }

    reserve
}

fn swap_labels(left: String, right: String) -> (String, String) {
    (right, left)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn add_exclamation(s: &mut String) {
    s.push('!');
}

fn add_marks(mut text: String) -> String {
    {
        let first = &mut text;
        first.push('!');
    }
    {
        let second = &mut text;
        second.push('?');
    }
    text
}

fn length_then_add(mut text: String) -> (usize, String) {
    let before_len = text.len();
    text.push('!');

    (before_len, text)
}

fn prefix(text: &str, end: usize) -> &str {
    &text[..end]
}

fn first_word(s: &str) -> &str {
    let idx = match s.find(' ') {
        Some(value) => value,
        None => return s,
    };

    &s[..idx]
}

fn inner_slice(values: &[i32]) -> &[i32] {
    &values[1..values.len() - 1]
}
