// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.
// 本题与上一题共用 `build.rs`。
// 你需要向 `build.rs` 添加代码，使本题和上一题都能正常工作。
//
// 执行 `rustlings hint tests8` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
