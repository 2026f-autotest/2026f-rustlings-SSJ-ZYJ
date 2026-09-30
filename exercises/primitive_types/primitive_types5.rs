// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.
// 对 `cat` 元组进行解构，使 println 能正常工作。
// 执行 `rustlings hint primitive_types5` 获取提示，或使用 watch 子命令中的 hint。

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;
    // 在这里填写你的模式。

    println!("{} is {} years old.", name, age);
}
