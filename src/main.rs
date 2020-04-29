mod chapter_3;
mod chapter_4;
mod chapter_5;
mod chapter_6;

fn main() {
    let some_value = Some(String::from("Hello, world"));
    let absent_value: Option<String> = None;

    println!("some_value: {:#?}", some_value);
    println!("absent_value: {:#?}", absent_value);

    let some_number = Some(10);

    if let Some(10) = some_number {
        println!("{:#?}", some_number);
    }
}
