#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle{
        width:30,
        height: 50,
    };
    println!("The area of rectangle is {} square pixels.",area(&rect));
    println!("Rect1 is {rect:#?}"); 
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}
