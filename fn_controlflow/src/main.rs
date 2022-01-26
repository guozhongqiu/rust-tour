fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);

    code_block();
    test_loop();
    test_while();
    test_for();
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn code_block() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}, x is {}", y, x);
}

fn test_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn test_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("while LIFTOFF!!!");
}

fn test_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
    
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("for LIFTOFF!!!");
}
