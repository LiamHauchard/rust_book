mod chapter_3;
mod chapter_4;

use chapter_4::first_word;

fn main() {
    let sentence = String::from("Hello world");

    let word = first_word(&sentence[..]);

    println!("{}", word);
}
