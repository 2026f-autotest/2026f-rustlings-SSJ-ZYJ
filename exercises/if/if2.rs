// if2.rs

// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.
// 第一步：让代码通过编译！
// 第二步：让 bar_for_fuzz 和 default_to_baz 测试通过！
// 执行 `rustlings hint if2` 获取提示，或使用 watch 子命令中的 hint。

pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    } else if fizzish == "literally anything" {
        "baz"
    } else {
        "1"
    }
}

// No test changes needed!
// 无需修改测试！
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}
