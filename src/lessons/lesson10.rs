// with structs
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn run() {
    // Doing it this way can cause an error if the parameters are put in the wrong way
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width, height)
    );

    // Using a tuple to pass parameters is more flexible and can prevent errors
    // because the order of parameters doesn't matter.
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    // Using a struct to pass parameters is the most flexible and readable way.
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
