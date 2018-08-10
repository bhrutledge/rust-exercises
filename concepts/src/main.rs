fn main() {
    function_expression();
    break_loop();
    ownership();
}

// https://doc.rust-lang.org/book/2018-edition/ch03-03-how-functions-work.html
fn function_expression() {
    let x = 5;
    let y = plus_one({ let x = 3; x });

    assert_eq!(x, 5);
    assert_eq!(y, 4);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// https://doc.rust-lang.org/book/2018-edition/ch03-05-control-flow.html
fn break_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        // TODO: match?
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

// https://doc.rust-lang.org/book/2018-edition/ch04-02-references-and-borrowing.html
fn ownership() {
    let s1 = String::from("Hello");
    let s2 = s1;
    let x = 5;
    let y = x;

    // println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    println!("x = {}", x);
    println!("y = {}", y);

    let mut s3 = take_and_return(s2);
    // println!("s2 = {}", s2);
    println!("s3 = {}", s3);

    mutate(&mut s3);

    {
        let r1 = &s3;
        println!("r1 = {}", r1);
    }

    borrow(&s3);
    println!("s3 = {}", s3);

    take(s3);
    // borrow(&s3);

    copy(y);
    println!("y = {}", y);
}

fn take_and_return(value: String) -> String {
    println!("Returning {}", value);
    value
}

fn take(value: String) {
    println!("Taking {}", value);
}

fn borrow(value: &String) {
    println!("Borrowing {}", value);
}

fn mutate(value: &mut String) {
    println!("Mutating {}", value);
    value.push_str(", world");
}

fn copy(value: i32) -> i32 {
    println!("Copied {}", value);
    value
}