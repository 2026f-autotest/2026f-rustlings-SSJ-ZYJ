// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.
// 你的任务是创建一个 `Vec`，其中包含与数组 `a` 完全相同的元素。
// 让代码通过编译并通过测试！
// 执行 `rustlings hint vecs1` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    // 普通数组。
    let v = // TODO: declare your vector here with the macro for vectors
// TODO：在这里使用向量宏声明你的向量。

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
