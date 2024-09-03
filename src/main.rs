#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

fn main() {
    let rect = Rectangle {
        width: 73.0,
        height: 35.0,
    };

    println!("The area is {}", rect_area(&rect));
}

fn rect_area(rect: &Rectangle) -> f32 {
    rect.width * rect.height
}
