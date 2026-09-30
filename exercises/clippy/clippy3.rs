// clippy3.rs
//
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.
// 这里还有几个简单的 Clippy 修复，帮助你了解它的用途。
//
// 执行 `rustlings hint clippy3` 获取提示，或使用 watch 子命令中的 hint。

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;

    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // 交换这两个值！
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
