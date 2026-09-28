// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint iterators4` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    // 完成此函数，使其返回 num 的阶乘。
    // 不要使用：
    // - `return`
    // 尽量不要使用：
    // - 命令式循环（for、while）
    // - 额外变量
    // 如果想挑战更高难度，不要使用：
    // - 递归
    // 执行 `rustlings hint iterators4` 获取提示。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
