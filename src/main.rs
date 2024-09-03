#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        self.width > another_rect.width && self.height > another_rect.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 73.0,
        height: 35.0,
    };
    let rect2 = Rectangle {
        width: 34.0,
        height: 48.0,
    };

    println!("The area is {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
