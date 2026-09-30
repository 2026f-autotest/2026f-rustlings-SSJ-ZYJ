// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.
// Rust 编译器需要知道如何检查传入的引用是否有效，以便在引用可能在使用前离开作用域时提醒程序员。
// 记住，引用是借用，不拥有自己的数据。那当所有者离开作用域时会怎样？
//
// 执行 `rustlings hint lifetimes1` 获取提示，或使用 watch 子命令中的 hint。

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
