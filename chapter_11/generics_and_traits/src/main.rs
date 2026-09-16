use generics_and_traits_lib::{NewArticle, Posts, Summary};

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(6, *largst(&v));

    let v: Vec<char> = vec!['1', '2', '3', '4', '5', '6'];
    assert_eq!('6', *largst(&v));

    let _ = Point { x: 1, y: 2 };
    let _ = Point { x: 1.0, y: 2.0 };
    // let _ = Point { x: 1, y: 2.0 }; mismatched x and y types
    let integer_points = Point::new(1, 2);
    // integer_points.distance(); method not found
    let float_points = Point::new(1.0, 2.0);
    assert_eq!(2, float_points.distance() as i32);

    let r = Reading {
        sensor: "telemperature",
        value: 3,
    };

    r.replace_value("error");

    let s = NewArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    notify(&s);

    assert_eq!(Some(10), latest_departure(&vec![1, 3, 10, 4, 9]));
}

fn largst<T>(list: &[T]) -> &T
where
    T: PartialOrd + Send,
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Point<f64> {
    fn distance(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Reading<T, U> {
    sensor: T,
    value: U,
}

impl<T, U> Reading<T, U> {
    fn replace_value<K>(self, new_value: K) -> Reading<T, K> {
        Reading {
            sensor: self.sensor,
            value: new_value,
        }
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

fn latest_departure<T: Ord + Copy>(items: &[T]) -> Option<T> {
    items.iter().copied().max()
}
