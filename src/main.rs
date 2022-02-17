// fn main() {
//     println!("Hello, world!");
// }

/**
 * rust 拷贝区别
 *
 * rust 二次复制不是浅拷贝 而是稀释首次声明的值，也就是无效，或者是移动了指针
 * rust 建议深克隆的时候，使用clone函数，
 *  当出现 clone 调用时，你知道一些特定的代码被执行而且这些代码可能相当消耗资源。你很容易察觉到一些不寻常的事情正在发生
 *
 *
 */
fn main() {
    let mut s = String::from("hello");
    s.push_str("sdadad");
    print!("{}", s);

    main_clone();

    main_copy();

    main2();

    main3();
    main4();
}

fn main_clone() {
    let s = String::from("11-23");
    let q = s.clone(); //堆上的数据 确实 被复制了
    print!("q={}, s={}", q, s)
}

fn main_copy() {
    let x = 5;
    let y = x; //已知大小，rust特点不需要clone函数
    println!("x = {}, y = {}", x, y);
}

fn main2() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
                   // 所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

fn main3() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}

// 引用
fn main4() {
    let s1 = String::from("hello world");

    let len = calculate_length2(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}
