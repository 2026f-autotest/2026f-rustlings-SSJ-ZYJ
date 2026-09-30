// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.
// Clippy 是一组用于分析代码的 lint 工具，可以帮助你发现常见错误并改进 Rust 代码。
//
// 对于这些题目，如果 Clippy 警告检查失败，代码就会编译失败；请根据输出中的建议完成题目。
//
// 执行 `rustlings hint clippy1` 获取提示，或使用 watch 子命令中的 hint。

use std::f32::consts::PI;

fn main() {
    let radius = 5.00f32;

    let area = PI * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
