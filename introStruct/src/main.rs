struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
 
    let mut user1 = User {
        active: true,
        username: String::from("user123"),
        email: String::from("random@gmail.com"),
        sign_in_count: 1,
    };
    
    let black = Color(0, 0, 0);
    
    let user2 = buildUser(String::from("user2@gmail.com"),String::from("user2"));
}

fn buildUser(email: String, username: String) -> User {
    User{
        active:true,
        username,
        email,
        sign_in_count: 1,
    }
}


