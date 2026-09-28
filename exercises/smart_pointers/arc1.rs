// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
// 本题给出一个包含 0 到 99 的 u32 向量 `numbers`：
// [0, 1, 2, ..., 98, 99]。
// 我们希望同时在 8 个不同线程中使用这些数字。
// 每个线程都要计算带有偏移量的每第八个数字之和。
// 第一个线程（偏移量 0）计算 0、8、16、……
// 第二个线程（偏移量 1）计算 1、9、17、……
// 第三个线程（偏移量 2）计算 2、10、18、……
// ……
// 第八个线程（偏移量 7）计算 7、15、23、……
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
// 由于使用了线程，数据必须是线程安全的，因此这里使用 Arc。
// 需要修改两个 TODO。
//
// 在第一个 TODO 注释处为 `shared_numbers` 填入一个值，
// 在第二个 TODO 注释处为 `child_numbers` 创建初始绑定，使代码通过编译。
// 尽量不要复制 `numbers` 向量！
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.
// 执行 `rustlings hint arc1` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

#![forbid(unused_imports)] // Do not change this, (or the next) line.
// 不要修改这一行（也不要修改下一行）。
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = // TODO
    // TODO：在第一个 TODO 注释处为 `shared_numbers` 填入一个值，
    // 并在第二个 TODO 注释处为 `child_numbers` 创建初始绑定。尽量不要复制 `numbers` 向量！
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = // TODO
        // TODO：在第二个 TODO 注释处为 `child_numbers` 创建初始绑定。
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
