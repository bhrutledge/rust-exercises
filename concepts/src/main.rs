fn main() {
    function_expression();
    break_loop();
}

fn function_expression() {
    let x = 5;
    let y = plus_one({ let x = 3; x });

    assert_eq!(x, 5);
    assert_eq!(y, 4);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

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
