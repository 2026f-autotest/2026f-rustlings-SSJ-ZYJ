// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.
// 此函数在你传入空字符串时拒绝生成要打印在名牌上的文本。
// 如果它能解释问题所在，而不是有时简单返回 `None`，会更好。
// 幸运的是，Rust 有一个与 `Result` 类似、可以表达错误条件的结构。让我们使用它！
//
// 执行 `rustlings hint errors1` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

pub fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        // 不允许空名称。
        None
    } else {
        Some(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            // 不要修改这一行。
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
