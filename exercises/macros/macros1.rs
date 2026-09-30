// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint macros1` 获取提示，或使用 watch 子命令中的 hint。

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
