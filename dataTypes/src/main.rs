fn main() {
    
    //int datatype
    let x = 10; //imutable
    let mut y = 20; //mutable
    const z: u32 = 101; //constant with unsigned

    //unsigned value is never in negative
    let mut w: u32 = 30; 
    //unsigned value can be in negative
    let mut v: i32 = 20;

    //float datatype
    let a = 10.10;
    let mut b = 20.20;
    const c: f32 = 40.40; //constant
    
    //numeric operation 
    let sum = 30+12; //add
    let difference = 30-11; //subtract
    let product = 4*4; //multiply
    let quotient = 40 / 2; //division
    let truncated = -5 /3; //result in -1
    let remainder = 10 % 4; //remainder

    //boolean
    let t = true;
    let f = false;

    //character
    let ch = 'z';

    //tuple
    let tup: (i32,f64,u8) = (49,4.4,3);

    //getting the value of tuple
    let (d,e,f) = tup;

    println!("The value of first element is {d}");

    let secondElement = tup.1;

    println!("The value of second element is {secondElement}");

    //array
    let g = [1,2,3,4,5];

    let h:[i32; 4] = [1,2,3,4];

    let i = [2;5]; //creates [2,2,2,2,2]

    //accessing the value of array
    let first = g[0];
    println!("The value of first element is {first}");
    
    let second = h[1];
    println!("Second Element is {second}");

    let last = i[4];
    println!("Last Element {last}");

}
