// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.
// 不改变函数签名，让代码通过编译！
//
// 执行 `rustlings hint strings1` 获取提示，或使用 watch 子命令中的 hint。

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string()
}
