// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint options1` 获取提示，或使用 watch 子命令中的 hint。

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
// 此函数返回冰箱里剩余的冰淇淋数量。
// 晚上 10 点之前有 5 份；到了晚上 10 点，有人会把它们全部吃掉，因此剩余为 0 :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    // 这里使用 24 小时制，因此晚上 10 点是 22，凌晨 12 点是 0。
    // 当 time_of_day > 23 时，Option 返回值应能妥善处理这种情况。
    // TODO：完成函数体——记得返回一个 Option！
    if time_of_day < 22 {
        Some(5)
    } else if time_of_day > 23 {
        None
    } else {
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        // TODO：修复这个测试。如何获取 Option 中包含的值？
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams.unwrap(), 5);
    }
}
