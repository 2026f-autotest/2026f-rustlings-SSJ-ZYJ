// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand for a hint.
// 使用元组索引访问 `numbers` 的第二个元素。
// 将访问第二个元素的表达式放在 ??? 处，使测试通过。
// 执行 `rustlings hint primitive_types6` 获取提示，或使用 watch 子命令中的 hint。

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    // 将下面的 ??? 替换为元组索引语法。
    let second = numbers.1;

    assert_eq!(2, second, "This is not the 2nd number in the tuple!")
}
