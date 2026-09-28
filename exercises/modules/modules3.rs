// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
// 可以使用 `use` 关键字将模块路径从任意模块，尤其是 Rust 标准库，引入当前作用域。
// 从 `std::time` 模块引入 SystemTime 和 UNIX_EPOCH。
// 如果能用一行完成，还可获得额外的代码风格分！
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint modules3` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

// TODO: Complete this use statement
// TODO：完成这条 use 语句。
use ???

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
