// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
// 在这个例子中，我们将浅尝 Rust 标准库中的不安全函数。
// 修复所有问号和 TODO，使测试通过。
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint tests6` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
/// `ptr` 必须包含一个拥有所有权的 `Foo` 盒子。
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    // 安全性依据：按照契约，`ptr` 包含一个拥有所有权的 `Foo` 盒子。
    // 我们只需根据该指针重新构造盒子。
    let mut ret: Box<Foo> = unsafe { ??? };
    todo!("The rest of the code goes here")
    // 其余代码将在这里补充。
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        // 我们传入的是一个拥有所有权的 `Foo` 盒子。
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
