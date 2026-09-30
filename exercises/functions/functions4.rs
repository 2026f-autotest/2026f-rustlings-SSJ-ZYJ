// functions4.rs
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a hint.
// 执行 `rustlings hint functions4` 获取提示，或使用 watch 子命令中的 hint。

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// (Don't worry about the function bodies themselves, we're only interested
// in the signatures for now. If anything, this is a good way to peek ahead
// to future exercises!)
// 这家商店正在促销：价格为偶数时减免 10 Rustbucks，价格为奇数时减免 3 Rustbucks。
// （暂时不用担心函数体；本题只关注函数签名。这也是提前了解后续题目的好机会！）

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
