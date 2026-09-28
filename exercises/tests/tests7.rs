// tests7.rs
//
// When building packages, some dependencies can neither be imported in
// `Cargo.toml` nor be directly linked; some preprocesses varies from code
// generation to set-up package-specific configurations.
//
// Cargo does not aim to replace other build tools, but it does integrate
// with them with custom build scripts called `build.rs`. This file is
// usually placed in the root of the project, while in this case the same
// directory of this exercise.
//
// It can be used to:
//
// - Building a bundled C library.
// - Finding a C library on the host system.
// - Generating a Rust module from a specification.
// - Performing any platform-specific configuration needed for the crate.
//
// When setting up configurations, we can `println!` in the build script
// to tell Cargo to follow some instructions. The generic format is:
//
//     println!("cargo:{}", your_command_in_string);
//
// Please see the official Cargo book about build scripts for more
// information:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// In this exercise, we look for an environment variable and expect it to
// fall in a range. You can look into the testcase to find out the details.
//
// You should NOT modify this file. Modify `build.rs` in the same directory
// to pass this exercise.
// 构建软件包时，有些依赖既不能导入 `Cargo.toml`，也不能直接链接；
// 还有一些预处理步骤会根据代码生成过程设置软件包专属配置。
//
// Cargo 并不打算替代其他构建工具，但它通过名为 `build.rs` 的自定义构建脚本与这些工具集成。
// 该文件通常放在项目根目录，本题中则位于本题所在目录。
//
// 它可以用于：
//
// - 构建捆绑的 C 库。
// - 查找主机系统上的 C 库。
// - 根据规范生成 Rust 模块。
// - 执行 crate 所需的任何平台特定配置。
//
// 设置配置时，可以在构建脚本中使用 `println!`，告诉 Cargo 遵循某些指令。
// 通用格式如下：
//
//     println!("cargo:{}", your_command_in_string);
//
// 更多信息请参阅 Cargo 官方关于构建脚本的文档：
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// 本题查找一个环境变量，并要求它处于指定范围内。
// 可以查看测试用例了解具体细节。
// 你不应修改本文件，而应修改同一目录下的 `build.rs` 来通过本题。
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint tests7` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
