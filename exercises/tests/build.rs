//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
//! 这是 tests7 和 tests8 共用的构建脚本。
//!
//! 你需要修改此文件，使两道题都通过。

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    // 在 tests7 中，需要设置名为 `TEST_FOO` 的环境变量。
    // 将内容打印到标准输出，让 Cargo 执行设置。
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
                    // 这个时间戳在这里有什么作用？
    let your_command = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // 在 tests8 中，需要启用 "pass" feature，使测试用例提前返回。
    // 补充命令，告诉 Cargo 如何启用它。
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}
