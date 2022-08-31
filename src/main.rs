#[derive(Debug)]
struct Rectangle { 
    width: u32,
    height: u32,
}

impl Rectangle {    // METHODS
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {     //ASSOCIATED FUNCTONS
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,   
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(3);

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}", rect1.can_hold(&rect3));

    // println!(
    //     "The area of the rectangle is {} square pixels.", rect1.area()
    // );

    println!("Square is {:#?}", sq);
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// struct AlwaysEqual;

// fn main() {
//     let mut user1 = User {
//         email: String::from("marcel@gmail.com"),
//         username: String::from("monqyClouds"),
//         active: true,
//         sign_in_count: 2,
//     };

//     user1.email = String::from("arthurs@gmailcom");

//     let user2 = User {
//         email: String::from("somto@gmail.com"),
//         ..user1
//     };

//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);

//     let subject = AlwaysEqual;
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
