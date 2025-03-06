fn main() {
    
    anotherFunction();
    parameterFunction(3);

    let y = five();
    println!("The value is {y}");

    let z = plusOne(6);
    println!("The value is {z}");

}

fn anotherFunction(){
    println!("This is not main function");
}

fn parameterFunction(x: i32){
    println!("The parameter is {x}");
}

fn five() -> i32{
    5
}

fn plusOne(x: i32) -> i32 {
    x+1
}


