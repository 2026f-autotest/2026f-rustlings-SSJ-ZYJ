// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
// Rust 中的 `unsafe` 是一种契约。
//
// 当 `unsafe` 标记在函数、trait 等项目声明上时，它会与该项目一起声明一个契约。
// 但契约内容不能只用一个关键字表达，因此需要在项目文档注释的 `# Safety` 部分手动说明。
//
// 当 `unsafe` 标记在花括号包围的代码块上时，它表示遵守了某个契约，例如指针参数有效，
// 或某个内存地址的所有权有效。与上面一样，你仍需在代码块注释中说明如何遵守该契约。
//
// 注意：所有注释都是为了提高代码的可读性和可维护性；Rust 编译器会把代码健全性的信任交给你自己！
// 如果无法证明代码的内存安全性和健全性，请退一步，改用安全代码。
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint tests5` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
/// `address` 必须包含对有效 `u32` 值的可变引用。
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    // TODO：补充下面代码块的安全说明，使其与代码行为及本函数契约一致。
    // 可以参考下面测试中的注释格式。
    unsafe {
        todo!("Your code goes here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        // 地址保证有效，并且包含对 `u32` 局部变量的唯一引用。
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
