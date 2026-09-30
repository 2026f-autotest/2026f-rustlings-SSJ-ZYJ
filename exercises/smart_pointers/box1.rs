// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
// 本题要实现的递归类型是 `cons list`，这是函数式编程语言中常见的数据结构。
// cons list 的每个元素包含两个部分：当前元素的值和下一个元素。
// 最后一个元素的值称为 `Nil`。
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
// 第 1 步：在枚举定义中使用 `Box`，使代码通过编译。
// 第 2 步：替换 `todo!()`，创建空的和非空的 cons list。
//
// Note: the tests should not be changed
// 注意：不应修改测试。
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.
// 执行 `rustlings hint box1` 获取提示，或使用 watch 子命令中的 hint。

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
    // TODO：在枚举定义中使用 `Box`，使代码通过编译。
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
