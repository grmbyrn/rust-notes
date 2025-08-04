pub fn run() {
    // s is not valid here, it's not yet declared
    let mut s = String::from("Hello"); // s is valid from this point forward (variable scope)
                                       // string type is allocated on the heap and not on the stack

    s.push_str(", world!");

    println!("{}", s); // This will print "Hello, world!"

    // do ..... // s is still valid

    let s1 = String::from("Hello");
    let s2 = s1.clone(); // s2 is a copy of s1, because i32 implements the Copy trait

    // print s2
    println!("{}", s2); // This will print 5

    // print s1
    println!("{}", s1); // This will also print 5, because s1 is still valid
} // the scope is now over, and s is no longer valid
