struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// Tuple structs without named fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit like structs without any field - seriously
struct AlwaysEqual;

fn main() {
    let mut user1 = User { // Make entire instance mut, only certain fields cant be mutable
        active: false, 
        username: String::from("Adam"), //Wrap in String else its &str - delibrately put String so struct owns its data, &str is just a reference
        email: String::from("adamlev@gmail.com"), // if we want to use &str we will need to specify lifetime
        sign_in_count: 2
    };

    user1.email = String::from("levineee@gmail.com");

    ///////////////////////////////////
    // Creating instance from other instances
    ///////////////////////////////////

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("newemail@gmail.com"),
        sign_in_count: user1.sign_in_count
    };
    
    // lot easier approach
    let user2 = User {
        email: String::from("newemail@gmail.com"),
        ..user1
    }; //cannot use user1 as a whole after that, cause user1 has been moved

    let user3 = User {
        email: String::from("newemail@gmail.com"),
        ..user1
    }; // see gave compile time error - move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait

    ///////////////////////////////////

    let black = Color(32, 43, -98);
    let origin = Point(0, 0, 0);

    println!("The coordinates of origin are: {0}, {1}, {2}", origin.0, origin.1, origin.2);

    ///////////////////////////////////

    let sub = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, // can only write username instead of username: username if param and value have same name
        email: email, 
        sign_in_count: 1,
    }
}
