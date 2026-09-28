// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.
// 在集合元素上执行操作时，迭代器非常重要。
// 本模块帮助你熟悉迭代器的结构，以及如何遍历可迭代集合中的元素。
//
// 填入 `???`，让代码通过编译。
//
// 执行 `rustlings hint iterators1` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = ???;   // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 4
    // TODO：第 1 步
    // TODO：第 2 步
    // TODO：第 3 步
    // TODO：第 4 步
}
