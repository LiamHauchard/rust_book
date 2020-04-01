pub fn _fibonaci(input: u32) -> u32 {
    match input {
        0 => 0,
        1 => 1,
        other => other + _fibonaci(other - 1),
    }
}

pub fn _celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}

pub fn _fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) / 1.8
}

pub fn days_of_christmas() {
    let days = vec![
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = vec![
        "and a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "12 Drummers Drumming",
    ];

    let first_gift = "A Partridge in a Pear Tree";

    for christmas_day in 0..12 {
        println!("On the {} day of Christmas", days[christmas_day]);
        println!("my true love sent to me:");

        if christmas_day == 0 {
            println!("{}", first_gift);
            println!();
            continue;
        }

        for day in (0..(christmas_day + 1)).into_iter().rev() {
            println!("{}", gifts[day]);
        }

        println!();
    }
}
