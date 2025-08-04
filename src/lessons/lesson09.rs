// structs in rust: used to create custom data types
// similar to tuples, but with named fields
// used to create more complex data types
// structs are immutable by default

// structs with name, email, is_active, age
struct User {
    name: String,
    email: String,
    is_active: bool,
    age: u8,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct is a struct with no fields and no values
#[derive(Debug)]
struct Unit;

pub fn run() {
    let mut user1 = User {
        name: String::from("John Doe"),
        email: String::from("doe@gmail.com"),
        is_active: true,
        age: 25,
    };

    user1.name = String::from("Jane Doe");

    // print all the values
    println!(
        "Name: {} \nEmail: {} \nIs Active: {} \nAge: {}",
        user1.name, user1.email, user1.is_active, user1.age
    );

    let user2 = build_user(String::from("Paul Smith"), String::from("smith@gmail.com"));

    println!(
        "Name: {} \nEmail: {} \nIs Active: {} \nAge: {}",
        user2.name, user2.email, user2.is_active, user2.age
    );

    // creating an instance of the struct
    let user3 = User {
        name: String::from("Alice Johnson"),
        email: String::from("johnson@gmail.com"),
        is_active: false,
        age: 40,
    };

    // create a new instance from another instance
    let user4 = User {
        name: String::from("Bob Brown"),
        email: user3.email,
        is_active: user3.is_active,
        age: user3.age,
    };

    // print all the values of user4
    println!(
        "Name: {} \nEmail: {} \nIs Active: {} \nAge: {}",
        user4.name, user4.email, user4.is_active, user4.age
    );

    // tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black Color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin Point: ({}, {}, {})", origin.0, origin.1, origin.2);

    // unit-like struct
    let unit = Unit;
    println!("Unit-like struct: {:?}", unit);
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        is_active: true,
        age: 18,
    }
}
