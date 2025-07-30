pub fn run() {
    // FUNCTIONS
    another_function(42, 'A');
    // This function is public and can be called from outside this module

    // Statements and Expressions
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // Return values from functions
    let x = sum_diff(5, 7);
    println!("sum and diff is: {:?}", x);
}

fn another_function(num: i32, letter: char) {
    // This function is private to this module
    println!(
        "Another function called with number: {} and char: {}",
        num, letter
    );
}

fn sum_diff(num1: i32, num2: i32) -> (i32, i32) {
    return (num1 + num2, num1 - num2);
    //println!("This line will never be executed");
}
