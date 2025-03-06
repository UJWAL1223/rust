fn main() {
    
    //not mutable can't change later 
    let x = 10;
    println!("The number is {x}");

    //mutable can be change
    let mut y = 10;
    println!("The number is {y}");
    y += 10;
    println!("The number is {y}");
    y = 30;
    println!("The number is {y}");

    //constant 
    const secondsInMinute: u32 = 60;
    println!("The number is {secondsInMinute}");

    //shadowing
    let z = 50;
    let z = z + 5;

    {
        let z = z + 30;

        println!("The value of z inside the scope is {z}");

    }

    println!("The value of z outside is {z}");



}
