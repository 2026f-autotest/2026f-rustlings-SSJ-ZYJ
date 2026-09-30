// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.
// 处理所有 TODO，使测试通过！
//
// 执行 `rustlings hint structs1` 获取提示，或使用 watch 子命令中的 hint。

struct ColorClassicStruct {
    // TODO: Something goes here
    // TODO：这里需要补充内容。
    red: u8,
    blue: u8,
    green: u8,
}

struct ColorTupleStruct(u8, u8, u8);
// TODO：这里需要补充内容。

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // let green =
        // TODO：实例化一个经典 C 风格结构体！
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // let green =
        // TODO：实例化一个元组结构体！
        let green = (0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        // let unit_like_struct =
        let unit_like_struct = UnitLikeStruct;
        // TODO：实例化一个类单元结构体！
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
