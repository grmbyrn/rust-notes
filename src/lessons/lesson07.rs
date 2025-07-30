pub fn run() {
    // Ownership and functions
    let i = 5;
    call_int(i);
    println!("AFTER CALLING THE FUNCTION, the value of i: {}", i);

    let s = String::from("Hello");
    call_string(s);

    println!("-----------------");

    let s1 = give_ownership();
    println!("s1: {}", s1);

    let s2 = String::from("Hello from main");

    let s3 = take_and_return_ownership(s2);
    println!("s3: {}", s3);

    println!("-----------------");

    let st1 = String::from("Hello");
    let len = calculate_length(&st1); // Pass a reference to the string
    println!("The length of '{}' is {}.", st1, len);

    println!("-----------------");

    let mut str1 = String::from("Hello");
    change_borrowed_value(&mut str1); // Pass a mutable reference to the string
    println!("After changing borrowed value: {}", str1);

    println!("-----------------");

    // Multiple references
    let mut str2 = String::from("Hello");

    {
        let r1 = &mut str2;
        r1.push_str(", world!"); // Modify the string through the mutable reference
    }

    let r2 = &mut str2;
    r2.push_str("|");

    println!("After multiple references: {}", str2);

    // Mutable and immutable references
    let mut t = String::from("Hello");

    let t1 = &t; // Immutable reference
    let t2 = &t; // Another immutable reference

    println!("{} {}", t1, t2); // Rust understands they are immutable and we're doing something with them.

    let t3 = &mut t; // Mutable reference

    println!("{}", t3); // If all were put together here, it would cause a compile-time error because you cannot have mutable and immutable references at the same time.

    println!("-----------------");

    let ref_value = no_dangle();
    println!("{}", ref_value);
}

// call int function
fn call_int(i: i32) {
    println!("call_int i: {}", i);
}

// call string function
fn call_string(i: String) {
    println!("call_string i: {}", i);
}

// function to give ownership of a string to another function
fn give_ownership() -> String {
    let some_string = String::from("Hello from give_ownership");
    some_string
}

// function to take and return ownership of a string
fn take_and_return_ownership(some_string: String) -> String {
    some_string
}

// Calculate the length of a string
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

// Change borrowed value
fn change_borrowed_value(s: &mut String) {
    s.push_str(", world!");
}

// Dangling references
fn no_dangle() -> String {
    let s = String::from("Hello, world!");
    s
}
