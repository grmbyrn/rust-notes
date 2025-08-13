/*
Methods in rust are similar to functions, but are defined within the context
of a struct (or an enum or a trait object). They are called by instances of the
struct, and their first parameter is always self, which represents the instance
of the struct. This following example demonstrates how to define a method for the
Rectangle struct that calculates the area of a rectangle.
*/

struct Rectangle {
    width: u32,
    height: u32,
}

// impl block is used to define methods for a struct
impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // methods with the same name as a field
    fn width(&self) -> bool {
        self.width > 0
    }

    // getter for height
    fn height(&self) -> u32 {
        self.height
    }

    // check if rectangle can hold another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
            || self.width > other.height && self.height > other.width
    }

    // associated function to define a square
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    let rect1 = Rectangle {
        width: 0,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // print the area
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // print the width if it is greater than 0, otherwise print 0
    if rect1.width() {
        println!("The width of the rectangle is {}.", rect1.width);
    } else {
        println!("The width of the rectangle is 0.");
    }

    // print the height
    println!("The height of the rectangle is {}.", rect1.height());

    // can rect1 hold rect2?
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // can rect3 hold rect2?
    println!("Can rect1 hold rect2? {}", rect3.can_hold(&rect2));

    // create a square
    let square = Rectangle::square(10);

    // calculate the area of the square
    println!("The area of the square is {} square pixels.", square.area());

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
