pub fn run() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let y = 2;
    println!("The value of y is: {}", y);

    {
        let y = 8;
        println!("The value of y in the inner scope is: {}", y);
    }

    let y = y + 2;
    println!("The value of y is: {}", y);

    // constants
    const MAX_POINTS: i32 = 100_000;
    println!("The maximum points is: {}", MAX_POINTS);
}
