// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
// 你的任务是替换 `??` 部分，使代码通过编译。
//
// 除标记的那一行外，不要修改其他任何代码行。
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint traits4` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
// 只能修改下一行。
fn compare_license_types(software: ??, software_two: ??) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
