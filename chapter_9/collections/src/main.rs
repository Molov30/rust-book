use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let _: Vec<i32> = Vec::new();
    let _ = vec![1, 2, 3];

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    println!("{:?}", v);

    let third: &i32 = &v[2];
    println!("Third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(val) => println!("Third element is {}", val),
        None => println!("There is no third element"),
    }

    for i in &mut v {
        *i *= 2
    }

    for i in v {
        println!("{}", i);
    }

    let v = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("text")),
    ];

    for i in v {
        println!("{:?}", i);
    }

    let _ = String::new();
    let mut s1 = String::from("is string 1");
    let data = "is string 2";
    let s2 = data.to_string();

    s1.push('\n');
    s1.push_str(&s2);
    println!("{}", s1);

    let s = String::from("Здравствуйте!");
    let s1 = &s[0..4];
    println!("{}", s1);

    println!("bytes: {}  chars: {:?}", s.bytes().len(), s.chars().count());

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let score = scores.get("Blue").copied().unwrap_or(0);
    assert_eq!(10, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("{:?}", scores);

    let inp = vec![
        (String::from("tea"), 3),
        (String::from("rice"), 2),
        (String::from("tea"), 4),
    ];

    let total = stock_totals(inp);
    assert_eq!(7, *total.get("tea").unwrap_or(&0));
    assert_eq!(2, *total.get("rice").unwrap_or(&2));

    let inp = vec![
        String::from("rust"),
        String::from("web"),
        String::from("rust"),
        String::from("cli"),
    ];
    let total = tag_count(inp, "rust");
    assert_eq!(2, total);

    assert_eq!("irst-fay apple-hay", pig_latin("first apple"));
}

fn stock_totals(records: Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut joined_records = HashMap::new();
    for (record, count) in records {
        let r = joined_records.entry(record).or_insert(0);
        *r += count
    }

    joined_records
}

fn tag_count(tags: Vec<String>, target: &str) -> usize {
    let mut count = 0;
    tags.iter().for_each(|t| {
        if t == target {
            count += 1
        }
    });

    count
}

fn pig_latin(text: &str) -> String {
    let mut res: Vec<String> = Vec::new();
    for word in text.split_whitespace().map(String::from) {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            let str: String;
            if matches!(first.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') {
                str = format!("{}{}-hay", first, chars.as_str(),);
            } else {
                str = format!("{}-{}ay", chars.as_str(), first);
            }
            res.push(str);
        };
    }

    res.join(" ")
}
