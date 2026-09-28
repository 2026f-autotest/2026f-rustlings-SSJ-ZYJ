// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.
// 让代码通过编译，但不能将宏移出模块！
//
// 执行 `rustlings hint macros3` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
