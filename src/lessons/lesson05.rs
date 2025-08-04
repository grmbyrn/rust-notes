pub fn run() {
    let number = 3;

    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    let condition = true;

    let n = if condition { 5 } else { 6 };
    println!("The value of n is: {}", n);

    // Nested if expressions
    let x = 11;

    if x % 2 == 0 {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);

        if x > 10 {
            println!("{} is also greater than 10", x);
        } else {
            println!("{} is not greater than 10", x);
        }
    }

    println!("-------------------------");

    // && || operators
    let a = 10;
    let b = 5;
    let c = 20;

    if a > b && b > c {
        println!("a is greater than b and b is greater than c");
    } else {
        println!("Condition with && not met");
    }

    if a > b || b < c {
        println!("At least one condition with || is met");
    } else {
        println!("Condition with || not met");
    }

    // Match: with enum we can define a type with a fixed set of values
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    // define coin
    let coin = Coin::Penny;

    // print value of coin
    println!("The value of the coin is: {} cents", value_in_cents(coin));

    // Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Breaks the loop and returns the value
        }
    };

    println!("The result of the loop is: {}", result);

    // While loop
    let mut count = 3;

    while count != 0 {
        println!("{}!", count);

        count -= 1;
        // wait for 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("Liftoff!");

    // For loop
    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    let s = "hello world";

    for c in s.chars() {
        println!("The value is: {}", c);
    }

    for number in (1..4).rev() {
        println!("The number is: {}", number);
    }

    println!("Go!");

    // FizzBuzz
    for number in 1..=100 {
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
    }
}
