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

#[derive(Debug)]
struct Rect2 {
    top_left: Point,
    height: f32,
    width: f32
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
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

    let sq2 = square(Point {x: 3.7, y: 1.4}, 3.0);

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.", area_rectangle(&rect1)
    );

    println!(
        "The area of the second square is {} square pixels.", area_square(&sq2)
    );

    println!("Square is {:#?}", sq);
    println!("Another Square is {:#?}", sq2)
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

fn area_rectangle(rectangle: &Rectangle) -> u32 { // nested destructuring
    let Rectangle {width, height} = rectangle;

    width * height
}

fn square(point: Point, lnt: f32) -> Rect2 {
    let Point {x, y} = point;

    let rectangle =  Rect2 {
        top_left: Point {x, y},
        height: lnt,
        width: lnt
    };

    return rectangle;
}

fn area_square(square: &Rect2) -> f32 {
    let Rect2 {width, height, top_left:_} = square;

    width * height
}

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
