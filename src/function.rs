fn add_value() -> isize {
    5
}

fn arg_value(x: isize, y: isize) -> isize {
    x + y;
}

fn main() {
    let get_value = add_value();
    println!("{}", get_value);
    println!("{}", arg_value(1, 2))
}
