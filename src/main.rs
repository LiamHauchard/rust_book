mod chapter_3;
mod chapter_4;
mod chapter_5;

use chapter_5::Rectangle;

fn main() {
    let rec = Rectangle::new(10, 10);

    println!("{:#?}", rec);
    println!("rec's area: {}", rec.area());
}
