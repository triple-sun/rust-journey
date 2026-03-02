pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
   pub fn area(&self) -> u32 {
        self.height * self.width
    }

    pub fn perimeter(&self) -> u32 {
        self.height * 2 + self.width * 2
    }

    pub fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.width > rect.width
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const TEST_RECTANGLE  : Rectangle = Rectangle{
            height: 30, width: 3
        };


    #[test]
    fn calculates_area() {
        assert_eq!(TEST_RECTANGLE.area(), 90);
    }

    #[test]
    fn calculates_perimeter() {
        assert_eq!(TEST_RECTANGLE.perimeter(), 66);
    }

    #[test]
    fn can_hold_smaller() {
        let smaller = Rectangle {
            height: 20, width: 1
        };

        assert!(TEST_RECTANGLE.can_hold(&smaller));
    }
}