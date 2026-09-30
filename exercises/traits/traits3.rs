// traits3.rs
//
// Your task is to implement the Licensed trait for both structures and have
// them return the same information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
// 你的任务是为两个结构体实现 `Licensed` trait，并让它们返回相同的信息，同时不要重复编写函数。
//
// 想一想可以向 `Licensed` trait 中添加什么。
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint traits3` 获取提示，或使用 watch 子命令中的 hint。

pub trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Some information")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line
                                   // 不要修改这两行。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
