fn main() {
    // TODO: struct instead of tuple
    let verses = [
        ("first", "a partridge in a pear tree"),
        ("second", "two turtle doves"),
        ("third", "three french hens"),
        ("fourth", "four calling birds"),
    ];

    // TODO: enumerate ala Python
    // TODO: print w/o newline
    // TODO: print w/o string literal
    for verse in 0..verses.len() {
        let day = verses[verse].0;
        println!("\nOn the {} day of Christmas my true love sent to me:", day);

        // TODO: Range vs. Slice
        for verse_lyric in (0..verse + 1).rev() {
            let lyric = verses[verse_lyric].1;

            if verse == 0 || (verse == 1 && verse_lyric == 1) {
                println!("{}", lyric);
            } else if verse_lyric == 0 {
                println!("and {}", lyric);
            } else {
                println!("{},", lyric);
            }
        }
    }
}
