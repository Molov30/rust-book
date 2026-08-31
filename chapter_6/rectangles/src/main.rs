#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct TeamStats {
    wins: u32,
    draws: u32,
    losses: u32,
}

struct ShelfItem {
    in_stock: u32,
    reserved: u32,
    minimum: u32,
}

struct Timer {
    planned_minutes: u32,
    elapsed_minutes: u32,
}

impl Timer {
    fn remaining(&self) -> u32 {
        if self.elapsed_minutes >= self.planned_minutes {
            return 0;
        }
        self.planned_minutes - self.elapsed_minutes
    }
}

struct TemperatureRange {
    min_c: i32,
    max_c: i32,
}

impl TemperatureRange {
    fn contains(&self, temperature: i32) -> bool {
        temperature >= self.min_c && temperature <= self.max_c
    }
}

struct MinutePack {
    base_minutes: u32,
    bonus_minutes: u32,
}

impl MinutePack {
    fn with_bonus(minutes: u32) -> Self {
        Self {
            base_minutes: minutes,
            bonus_minutes: minutes / 10,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rectangle is {} square pixels", rect1.area());
    assert!(rect1.can_hold(&Rectangle {
        width: 20,
        height: 10
    }));

    let team_stat = TeamStats {
        wins: 2,
        draws: 4,
        losses: 3,
    };
    assert_eq!(10, league_points(&team_stat));

    assert_eq!(
        true,
        needs_restock(&ShelfItem {
            in_stock: 10,
            reserved: 11,
            minimum: 3
        })
    );
    assert_eq!(
        true,
        needs_restock(&ShelfItem {
            in_stock: 6,
            reserved: 2,
            minimum: 5
        })
    );
    assert_eq!(
        false,
        needs_restock(&ShelfItem {
            in_stock: 10,
            reserved: 2,
            minimum: 3
        })
    );

    println!("rect1 is {:?}", rect1);

    assert_eq!(
        15,
        Timer {
            planned_minutes: 25,
            elapsed_minutes: 10
        }
        .remaining()
    );

    assert_eq!(
        0,
        Timer {
            planned_minutes: 10,
            elapsed_minutes: 10
        }
        .remaining()
    );

    assert!(
        TemperatureRange {
            min_c: 10,
            max_c: 20
        }
        .contains(10)
    );
    assert!(
        !TemperatureRange {
            min_c: 12,
            max_c: 20
        }
        .contains(9)
    );

    let pack = MinutePack::with_bonus(30);
    assert_eq!(30, pack.base_minutes);
    assert_eq!(3, pack.bonus_minutes);

    let pack = MinutePack::with_bonus(6);
    assert_eq!(6, pack.base_minutes);
    assert_eq!(0, pack.bonus_minutes);
}

fn league_points(stat: &TeamStats) -> u32 {
    stat.wins * 3 + stat.draws
}

fn needs_restock(item: &ShelfItem) -> bool {
    if item.reserved >= item.in_stock {
        return true;
    }

    let available = item.in_stock - item.reserved;
    available < item.minimum
}
