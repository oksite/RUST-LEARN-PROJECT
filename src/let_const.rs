const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn fu_let() {
    let x = 10;
    println!("x {}", x)
}

fn change_let() {
    let x = 200;
    let x = x - 100;
    println!("x {} ", x)
}

fn change_value_let() {
    let mut x = 10; //可变
    println!("{}", x);
    x = 100;
    println!("value {} ", x);
}

fn fn_const() {
    println!("{}", THREE_HOURS_IN_SECONDS)
}

fn let_spaces() {
    let spaces = "";
    let spaces = spaces.len();
    println!("{}", spaces)
}

// fn error_let() {
//     let mut name = "oksite"; //remove this `mut`:
//     let name = name.len();
//     println!("{}", name)
// }

fn main() {
    fu_let();
    change_let();
    change_value_let();
    fn_const();
    let_spaces()
}
