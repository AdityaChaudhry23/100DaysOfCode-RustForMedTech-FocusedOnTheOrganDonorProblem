
// Defining a Struct in Rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Defining a Tuple 
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Defining a Unit like Struct
struct AlwaysEqual;

fn main() {
    // Creating an instance of that Struct
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

     user1.email = String::from("anotheremail@example.com"); //  If the instance is mutable, 
     // we can change a value by using the dot notation and assigning into a particular field. 

     // Creating Instance from Other Instances with struct update syntax
     // let user2 = User {
     //   active: user1.active,
     //   username: user1.username,
     //   email: String::from("another@example.com"),
     //   sign_in_count: user1.sign_in_count,
    //}; 

    // Or
     let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit Like Struct
    let subject = AlwaysEqual;
     
}
