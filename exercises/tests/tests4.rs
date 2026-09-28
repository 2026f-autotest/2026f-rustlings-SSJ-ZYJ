// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.
// 确保我们测试的是正确的条件！
//
// 执行 `rustlings hint tests4` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    // 只能修改测试函数本身。
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        // 本测试应检查矩形是否与传给构造函数的尺寸相同。
        let rect = Rectangle::new(10, 20);
        assert_eq!(???, 10); // check width
        assert_eq!(???, 20); // check height
    }

    #[test]
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        // 本测试应检查使用负宽度创建矩形时程序是否会 panic。
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        // 本测试应检查使用负高度创建矩形时程序是否会 panic。
        let _rect = Rectangle::new(10, -10);
    }
}
