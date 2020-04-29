#[derive(Debug)]
pub struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    pub fn new(height: u32, width: u32) -> Rectangle {
        Rectangle { height, width }
    }

    pub fn area(&self) -> u32 {
        self.height * self.width
    }

    pub fn can_hold(&self, rec: &Rectangle) -> bool {
        self.height > rec.height && self.width > rec.width
    }
}
