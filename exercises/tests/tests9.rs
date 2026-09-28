// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.
// Rust 非常擅长与 C/C++ 及其他静态编译语言共享 FFI 接口，甚至可以在代码内部完成链接！
// 它通过 extern 代码块实现，就像下面的代码一样。
//
// `extern` 关键字后的短字符串表示外部导入函数遵循的 ABI。本题使用 "Rust"，
// 其他变体还包括标准 C ABI 的 "C" 和 Windows ABI 的 "stdcall"。
//
// 外部导入函数声明在 extern 代码块中，以分号而不是花括号标记签名结束。
// 可以使用属性修改这些函数声明的链接行为，例如 `#[link_name = ".."]` 可以修改实际符号名。
//
// 如果希望将符号导出到链接环境，也可以在具有相同 ABI 字符串的函数定义前使用 `extern` 关键字。
// Rust 函数的默认 ABI 就是 "Rust"，因此如果要链接纯 Rust 函数，可以省略整个 extern 部分。
//
// Rust 默认会像 C++ 一样重整符号名。若要让函数可以按名称访问，可以使用 `#[no_mangle]` 属性。
//
// 本题要求测试用例能够调用 Foo 模块中的 `my_demo_function`。
// `my_demo_function_alias` 是 `my_demo_function` 的别名，因此测试中的两行代码应调用同一个函数。
// 除了添加两行属性外，不应修改任何现有代码。

// I AM NOT DONE

extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
    // 没有 `extern` 就等价于 `extern "Rust"`。
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        // 默认情况下，外部导入的函数是不安全的，因为其他语言的来源不受信任。
        // 你可以将它们包装在安全的 Rust API 中，减轻调用者的负担。
        //
        // 安全性依据：我们知道这些函数是安全 Rust 函数的别名。
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
