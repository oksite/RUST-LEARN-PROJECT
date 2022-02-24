// struct User {
//     name: $str,
//     email: $str,
//     age: usize,
//     active: bool
// }

// fn main() {
//     const user = User {
//         name: "okie",
//         email: "oksite@yeah.net",
//         age:20,
//         active: true
//     }
//     println!("{}", user)
// }

// struct User {
//     name: str,
//     email: str,
//     age: usize,
//     active: bool,
// }

// fn main() {
//     let user = User {
//         name: String::from("someone@example.com"),
//         email: String::from("someone@example.com"),
//         age: 20,
//         active: true,
//     };

//     println!("{}", user)
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(mut email: &str, mut username: &str) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     // let user1 = User {
//     //     email: String::from("someone@example.com"),
//     //     username: String::from("someusername123"),
//     //     active: true,
//     //     sign_in_count: 1,
//     // };

//     const a = build_user("someone@example.com", "someone");

//     println!("{}", a)
// }

/**
 * 计算长方形大小
 */
// fn main() {
//     let width = 30;
//     let height = 50;
//     println!("this is value {}", get_area(width, height))
// }

// fn get_area(width: i32, height: i32) -> i32 {
//     width * height
// }

/**
 * 元组
 */

// fn main() {
//     let value = (30, 50);
//     println!("this value is {}", get_value(value))
// }

// fn get_value(value: (i32, i32)) -> i32 {
//     value.0 * value.1
// }

/**
 * 结构
 */

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let info = Rectangle {
        width: 30,
        height: 50,
    };

    println!("this value {}", get_area(&info))
}

fn get_area(info: &Rectangle) -> u32 {
    info.width * info.height
}
