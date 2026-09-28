// if1.rs
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.
// 执行 `rustlings hint if1` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
// 完成此函数，使其返回较大的数字！
// 不要使用：
// - 其他函数调用
// - 额外变量
}

// Don't mind this for now :)
// 这部分暂时不用在意 :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
