// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.
// 在 ??? 处从数组 a 中取出切片，使测试通过。
// 执行 `rustlings hint primitive_types4` 获取提示，或使用 watch 子命令中的 hint。

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..=3];

    assert_eq!([2, 3, 4], nice_slice)
}
