// errors5.rs
//
// This program uses an altered version of the code from errors4.
//
// This exercise uses some concepts that we won't get to until later in the
// course, like `Box` and the `From` trait. It's not important to understand
// them in detail right now, but you can read ahead if you like. For now, think
// of the `Box<dyn ???>` type as an "I want anything that does ???" type, which,
// given Rust's usual standards for runtime safety, should strike you as
// somewhat lenient!
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The Box is declared as of type Box<dyn Trait> where Trait is
// the trait the compiler looks for on any value used in that context. For this
// exercise, that context is the potential errors which can be returned in a
// Result.
//
// What can we use to describe both errors? In other words, is there a trait
// which both errors implement?
// 这段程序使用了 errors4 中代码的一个修改版本。
//
// 本题会用到课程后面才会介绍的概念，例如 `Box` 和 `From` trait。
// 现在不必完全理解它们；如果愿意，也可以提前阅读相关内容。
// 目前可以把 `Box<dyn ???>` 理解为“我想要任何实现了 ??? 的类型”，
// 按照 Rust 通常的运行时安全标准来看，这个类型限制相当宽松！
//
// 简而言之，这里使用 Box 是因为你想拥有一个值，但只关心它实现了某个特定 trait。
// Box 会声明为 `Box<dyn Trait>`，其中 Trait 是编译器要求上下文中的值实现的 trait。
// 在本题中，这个上下文就是 Result 可能返回的错误。
//
// 我们可以用什么来描述这两个错误？换句话说，是否存在两个错误都实现的 trait？
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint errors5` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
// TODO：更新 `main()` 的返回类型，使代码通过编译。
fn main() -> Result<(), Box<dyn ???>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.
// 不要修改下面这一行之后的任何内容。

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
// 这段代码是 `CreationError` 实现 `error::Error` 所必需的。
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
