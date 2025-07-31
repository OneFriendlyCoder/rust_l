struct User {
    username: String,           //String is used rather than &str because we want to own the dataso that it stays valid till the struct is valid, not just borrow it
    email: String, 
    sign_in_count: u64,
    active: bool,
}

fn main() {

    let mut user1 = User{       //the whole instance is mutable
        username: String::from("helloworld"),
        email: String::from("helloworld@gmail.com"),
        sign_in_count: 10,
        active: true,
    };

    user1.username = String::from("hellworld1");
    let user2 = build_user(String::from("helloworld2@gmail.com"), String::from("helloworld2"));             // we have to convert the string literal to String

    //creating User instance from other instance with struct update syntax
    let user3 = User{
        email: String::from("helloworld3@gmail.com"),
        username: String::from("helloworld3"),
        ..user2 //using the values from user1 for the rest of the fields        
    };


    println!("Username: {}", user3.sign_in_count);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 100,
    }
}