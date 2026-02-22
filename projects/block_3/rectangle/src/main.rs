struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn perimeter(&self) -> u32 {
        self.height * 2 + self.width * 2
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.width > rect.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 123,
        height: 456,
    };

    let rect2 = Rectangle {
        width: 234,
        height: 567,
    };

    println!("Rectangle 1 area is {}", rect1.area());
    println!("Rectangle 2 area is {}", rect2.area());

    println!("Rectangle 1 perimeter is {}", rect1.perimeter());
    println!("Rectangle 2 perimeter is {}", rect2.perimeter());

    println!("Rectangle 1 can hold rectangle 2: {}", rect1.can_hold(&rect2));
    println!("Rectangle 2 can hold rectangle 1: {}", rect2.can_hold(&rect1));
}
