const VERSES: usize = 4;

const DAYS: [&str; VERSES] = [
    "first",
    "second",
    "third",
    "fourth",
];

const GIFTS: [&str; VERSES] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
];

fn main() {
    for (verse, &day) in DAYS.iter().enumerate() {
        print_verse(verse, day);
    }
}

fn print_verse(verse: usize, day: &str) {
    // TODO: Print on one line
    println!("\nOn the {} day of Christmas my true love sent to me:", day);

    // Print lines for this verse in reverse order, with proper punctuation
    let gifts = &GIFTS[..=verse];
    for (line, gift) in gifts.iter().rev().enumerate() {
        print_line(verse, line, gift);
    }
}

fn print_line(verse: usize, line: usize, gift: &str) {
    // TODO: One print w/ conditional format
    if verse < 2 && line == 0 {
        // No comma for first line of first two verses
        println!("{}", gift);
    } else if verse == line {
        // Last line
        println!("and {}", gift);
    } else {
        println!("{},", gift);
    }
}
