// move_semantics2.rs
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.
// 执行 `rustlings hint move_semantics2` 获取提示，或使用 watch 子命令中的 hint。

// Expected output:
// vec0 has length 3 content `[22, 44, 66]`
// vec1 has length 4 content `[22, 44, 66, 88]`
// 预期输出：
// vec0 长度为 3，内容为 `[22, 44, 66]`
// vec1 长度为 4，内容为 `[22, 44, 66, 88]`

fn main() {
    let mut vec0 = Vec::new();

    // Do not move the following line!
    // 不要移动下面这一行！
    let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    // 不要修改下面这一行！
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
}
