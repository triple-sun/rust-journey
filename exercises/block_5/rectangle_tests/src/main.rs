use rectangle_tests::Rectangle;

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
