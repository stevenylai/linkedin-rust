#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn scale(&mut self, ratio: f64) {
        self.height *= ratio;
        self.width *= ratio;
    }
    fn get_area(&self) -> f64 {
        self.height * self.width
    }
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle{
            width,
            height,
        }
    }
}

fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}