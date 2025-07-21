#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn test () {
    let rec = Rectangle {
        width: 30,
        height: 50
    };
    let area = cal_area(&rec);
    println!("The area of the rectangle is {} square pixels.", area);
    println!("The rec is : {:#?}", rec);
    dbg!(&rec);
}

fn cal_area (rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

pub fn test2 () {
    let rec = Rectangle {
        width: 30,
        height: 50
    };
    println!("The area of the rectangle is {} square pixels.", rec.cal_area());
    println!("The rec is : {:#?}", rec);
}

impl Rectangle {
    fn cal_area (&self) -> u32 {
        return self.width * self.height;
    }
}

pub fn test3 () {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

impl Rectangle {
    fn can_hold (&self, other: &Rectangle) -> bool {
        return self.width>=other.width && self.height>=other.height;
    }
}
