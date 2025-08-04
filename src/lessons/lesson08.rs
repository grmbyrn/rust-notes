// Slices in Rust are a reference to a contiguous sequence of elements in a
// collection. They are a view into the original collection, and do not store any
// data themselves. Slices are used to give a part of a collection to a function
// or to iterate over a part of collection.

// Slice syntax: &[T]
// T is the type of the elements in the collection.
// & is a reference to the collection.

pub fn run() {
    // first example: slice of an array of characters
    let chars: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let slice1: &[char] = &chars[1..3]; // slice from index 1 to 3 (exclusive)
    println!("Slice of characters: {:?}", slice1);

    // second example: slice of a vector of integers
    // vectors are resizable arrays
    let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
    let slice2: &[i32] = &vec[3..4];
    println!("Slice of vector: {:?}", slice2);

    // third example: slice for strings
    let s: String = String::from("Hello world");
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("{:?}", hello);
    println!("{:?}", world);

    // Ranges shortcut for slices
    let s2: String = String::from("Graeme");

    // shortcut for initial index
    let slice3: &str = &s2[0..3]; // equivalent to &s2[..3]
    println!("{:?}", slice3);
    let slice4: &str = &s2[..3];
    println!("{:?}", slice4);

    // shortcut for final index
    let len = s2.len();
    let slice5: &str = &s2[3..len]; // equivalent to &s2[3..]
    println!("{:?}", slice5);
    let slice6: &str = &s2[3..]; // equivalent to &s2[3..]
    println!("{:?}", slice6);

    // shortcut for both the initial and final index
    let slice7 = &s2[0..len];
    println!("{:?}", slice7);
    let slice8 = &s2[..]; // equivalent to &s2[0..len]
    println!("{:?}", slice8);

    // Exercise: get the first word of a string (no slices)
    let s3 = String::from("Hello world");
    let word = first_word(&s3);
    let s3 = first_word_string(&s3); // using the function to get the first word as a string slice
    println!("The s is = {}", s3);
    println!("The first word is = {}", word);
    println!("The first word is = {}", &s3[0..word]); // using the index to get the first word as a string slice

    // string literals
    let s4 = "second word1";
    let word2 = first_word_string(&s4); // using the function to get the first word as a string slice
    println!("The first word of the string literal is = {}", word2);
}

// Function to get the first word of a string
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert the string to bytes

    // iterate through the bytes and return the index of the first space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // if the byte is a space
            return i; // return the index
        }
    }
    // if no space is found, return the length of the string
    s.len() // return the length of the string
}

// Function to get the first word of a string
fn first_word_string(s: &str) -> &str {
    let bytes = s.as_bytes(); // convert the string to bytes

    // iterate through the bytes and return the index of the first space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // if the byte is a space
            return &s[0..i]; // return the index
        }
    }
    // if no space is found, return the length of the string
    &s[..] // return the whole string as a slice
}
