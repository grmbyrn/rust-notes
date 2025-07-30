struct Person {
    name: String,
    age: u8,
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub fn run() {
    // DATA TYPES
    // Scalar types
    // - integers

    // Length       Signed      Unsigned
    // - 8-bit      i8          u8
    // - 16-bit     i16         u16
    // - 32-bit     i32         u32
    // - 64-bit     i64         u64
    // - 128-bit    i128        u128
    // - arch       isize       usize

    let small_number: u8 = 255;
    let big_number: u128 = 123456789012434;
    let small_number2: i8 = -127;
    let big_number2: i128 = -12345678901234;

    println!("Small number: {}", small_number);
    println!("Big number: {}", big_number);
    println!("Small number: {}", small_number2);
    println!("Big number: {}", big_number2);
    println!("----------------------------------");

    // Numeral System   Description                         Example
    // Decimal          Base 10, common form                1.0, 2.5, -3.14
    // Hexadecimal      Base 16, prefixed with 0x           0xff
    // Octal            Base 8, prefixed with 0o            0o77
    // Binary           Base 2, prefixed with 0b            0b1111_0000
    // Byte (u8 only)   ASCII characters, prefixed with b   b'A', b'Z'

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("Decimal: {}", decimal);
    println!("Hexadecimal: {}", hex);
    println!("Octal: {}", octal);
    println!("Binary: {}", binary);
    println!("Byte: {}", byte);

    println!("----------------------------------");

    // - floating-point numbers
    let x = 2.5; // f64 by default
    let y: f32 = 3.0; // f32 explicitly
    println!("x: {}, y: {}", x, y);

    // - numeric operations
    let sum = x + y as f64; // casting y to f64
    let difference = x - y as f64;
    let product = x * y as f64;
    let quotient = x / y as f64;
    let remainder = x % y as f64;
    println!(
        "Sum: {}, Difference: {}, Product: {}, Quotient: {}, Remainder: {}",
        sum, difference, product, quotient, remainder
    );
    println!("----------------------------------");

    // - booleans
    let t = true; // implicit declaration
    let f: bool = false; // explicit declaration
    println!("Boolean t: {}, f: {}", t, f);

    //if
    if t {
        println!("t is true");
    } else {
        println!("t is false");
    }

    let not_t = !t; // negation
    println!("Negation of t: {}", not_t);

    println!("----------------------------------");

    // - characters
    let c = 'z'; // single character
    let z: char = 'Z'; // explicit declaration
    let emoji = '😃'; // emoji character
    println!("Character c: {}, z: {}, emoji: {}", c, z, emoji);

    // Compound types
    // - tuples
    let tup: (i32, f64, char) = (500, 6.4, 'a'); // tuple with different types
    let (x, y, z) = tup; // destructuring the tuple
    println!("Tuple values: x: {}, y: {}, z: {}", x, y, z);

    // Accessing tuple elements
    let first = tup.0; // accessing first element
    let second = tup.1; // accessing second element
    let third = tup.2; // accessing third element
    println!(
        "Tuple elements: first: {}, second: {}, third: {}",
        first, second, third
    );
    println!("----------------------------------");

    // - arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // array of fixed size

    let first = arr[0]; // accessing first element
    let second = arr[1]; // accessing second element
    let third = arr[2]; // accessing third element
    println!(
        "Array elements: first: {}, second: {}, third: {}",
        first, second, third
    );

    for element in arr.iter() {
        println!("Array element: {}", element);
    }
    println!("Array length: {}", arr.len());

    println!("----------------------------------");

    // Custom types
    // - structs
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    }; // creating an instance of Person struct
    println!("Person name: {}, age: {}", person.name, person.age);

    // - enums
    let light = TrafficLight::Red; // creating an instance of TrafficLight enum
    match light {
        TrafficLight::Red => println!("Traffic light is Red"),
        TrafficLight::Yellow => println!("Traffic light is Yellow"),
        TrafficLight::Green => println!("Traffic light is Green"),
    }
    println!("----------------------------------");
}
