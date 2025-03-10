fn main() {
    
    let s1 = givesOwnership();

    let s2 = String::from("Hello");

    let s3 = takesAndGivesOwnership(s2);

}

fn givesOwnership() -> String {
    let someString = String::from("yours");

    someString
}

fn takesAndGivesOwnership(aString: String) -> String{

    aString
}
