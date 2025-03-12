fn main(){

    let x: i8 = 7;
    let y: Option<i8> = None;

    let sum = x + y.unwrap_or(0);


}