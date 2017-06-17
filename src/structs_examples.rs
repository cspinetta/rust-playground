
#[derive(Debug)]
pub struct Rectangle {
    pub length: u32,
    pub width: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.length * self.width
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.length >= other.length
    }
}
