// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.
// 只能通过重新排列 `main()` 中的代码行让它通过编译，不能添加、修改或删除任何代码行。
// 执行 `rustlings hint move_semantics5` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

fn main() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut x;
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}
