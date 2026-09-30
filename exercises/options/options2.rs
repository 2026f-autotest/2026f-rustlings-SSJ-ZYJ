// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint options2` 获取提示，或使用 watch 子命令中的 hint。

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        // TODO：将这里改为 if let 语句，使其值为 "Some" 类型。
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // TODO：将这里改为 while let 语句——记住，vector.pop 还会再添加一层 Option<T>。
        // 你可以在 while let 和 if let 中嵌套多个 `Option`。
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
