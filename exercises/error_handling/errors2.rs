// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.
// 假设我们正在编写一个可以用代币购买物品的游戏。每件物品售价 5 个代币，每次购买还要支付 1 个代币的手续费。
// 玩家输入想购买的物品数量，`total_cost` 函数会计算所需总代币数。
// 但由于数量来自用户输入，我们得到的是字符串——用户可能输入任何内容，而不只是数字！
//
// 目前，这个函数完全没有处理错误情况（成功情况也处理得不正确）。
// 我们希望：对非数字字符串调用 `parse` 时会返回 `ParseIntError`，此时应立即从本函数返回该错误，
// 不要继续进行乘法和加法。
//
// 至少有两种正确实现方式，其中一种会短得多！
//
// 执行 `rustlings hint errors2` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
