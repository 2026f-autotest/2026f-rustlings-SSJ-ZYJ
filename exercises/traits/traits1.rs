// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
// 现在开始实现 trait！你的任务是为 `String` 类型实现 `AppendBar` trait。
// `AppendBar` trait 只有一个函数，用于给实现该 trait 的对象追加 "Bar"。
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint traits1` 获取提示，或使用 watch 子命令中的 hint。

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    // TODO：为 `String` 类型实现 `AppendBar`。
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
