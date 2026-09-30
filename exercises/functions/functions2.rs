// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.
// 执行 `rustlings hint functions2` 获取提示，或使用 watch 子命令中的 hint。

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
