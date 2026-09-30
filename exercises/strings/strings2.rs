// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.
// 不改变函数签名，让代码通过编译！
//
// 执行 `rustlings hint strings2` 获取提示，或使用 watch 子命令中的 hint。

fn main() {
    let word = String::from("green"); // Try not changing this line :)
                                      // 尽量不要修改这一行 :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
