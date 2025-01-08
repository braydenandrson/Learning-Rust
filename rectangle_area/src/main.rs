struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    /* Attempt 1
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    */
    /* Attempt 2
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area(rect1));
    */

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

/* Attempt 1
fn area(width: u32, height: u32) -> u32 {
    return width * height;
}
*/
/* Attempt 2
fn area(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}
*/
fn area( rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}
