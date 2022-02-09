/**
 * rust 数据类型 跟js一样，分为常用类型，特殊类型
 *
 * rust简称： 标量（scalar）和复合（compound） 一共六种数据类型
 *
 * 标量（scalar）:  整型、浮点型、布尔类型和字符类
 * 复合类型: 元组（tuple）和数组（array）
 *
 */

/**
* i 有负数
  u 正数
  usize 根据系统类型长度 > 0
  isize 根据系统类型长度 < 0 | > 0
*/
fn init_type() {
    let max_name: u32 = 39;
    let max_age: u64 = 10;
    let max_value: i16 = -10;

    let max_unkown: usize = 2000000002323;
    let max_inkown: isize = -2000000002323;

    println!("{}", max_name);
    println!("{}", max_age);
    println!("{}", max_value);
    println!("{}", max_unkown);
    println!("{}", max_inkown);
}

/**
 * f32 是单精度浮点数
 * f64 是双精度浮点数
 */
fn float_type() {
    let x: f64 = 3.20;
    let y: f32 = 4.0;
    println!("{}", x);
    println!("{}", y);

    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // 结果为 0

    // 取余
    let remainder = 43 % 5;

    println!("{}", sum);
    println!("{}", difference);
    println!("{}", product);
    println!("{}", quotient);
    println!("{}", floored);
    println!("{}", remainder);
}

fn bool_type() {
    let t = true;
    let f: bool = false; // 显式指定类型注解

    println!("{}", t);
    println!("{}", f)
}

fn char_type() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", c);
    println!("{}", z);
    println!("{}", heart_eyed_cat)
}

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}", tup.0);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

fn arr_type() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("{}", months[0]);
    println!("{}", months[1]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{}", a[0]);
}

fn main() {
    init_type();
    float_type();
    bool_type();
    char_type();
    tuple_type();
    arr_type()
}
