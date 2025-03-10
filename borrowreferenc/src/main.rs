fn main() {

    let mut s = String::from("Hello");

    change(&mut s);

}

fn change(modstring: &mut String){

    modstring.push_str(" World");
}

