// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
// 本题探索 Cow（即 Clone-On-Write，写时克隆）类型。
// Cow 是一种写时克隆智能指针，可以封装借用数据并提供不可变访问，
// 只有在需要修改或取得所有权时才会延迟克隆数据。
// 该类型通过 Borrow trait 支持通用的借用数据。
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
// 本题用于说明将数据传递给 Cow 时会发生什么。
// 在 TODO 标记处检查 Cow::Owned(_) 和 Cow::Borrowed(_)，修复单元测试。
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.
// 执行 `rustlings hint cow1` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            // 如果数据尚未拥有所有权，则克隆到向量中。
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        // 因为需要修改 `input`，所以会发生克隆。
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        // 因为不需要修改 `input`，所以不会发生克隆。
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            // TODO
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        // 也可以不加 `&` 直接传入 `slice`，这样 Cow 会直接拥有它。
        // 这种情况下不会发生修改，因此也不会克隆；但结果仍然是拥有所有权的，
        // 因为它从未被借用或修改。
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        // 发生修改时也是如此。这时 `to_mut()` 返回的引用仍然指向之前的同一份数据。
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
        }
    }
}
