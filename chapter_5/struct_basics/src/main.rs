struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, 
        email, // because parameter and struct fields match, we can rewrite like so
        sign_in_count: 1,
    }
}

fn main() {  
    let email: String = String::from("dave@dave.com");
    let username: String = String::from("dave_dave_dave");

    let mut user1 = User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    };
    // dot notation is used to retrieve attributes
    // we can update this one as we set the struct as mutable
    user1.email = String::from("anotheremail@example.com");  

    // using our function, commented out to avoid compiler error due to moved value
    // let user_1_5 = build_user(user1.email, user1.username);

    // we can use attributes from user1 in user2
    let user2 = User {
        email: String::from("and_anotheremail@example.com"),
        // username: user1.username,
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
        // shorthand is below
        ..user1
    };

    // active and sign_in_count have the copy attribute but username does not so the below print statement throws an error
    // println!("{}", user1.username)
}
