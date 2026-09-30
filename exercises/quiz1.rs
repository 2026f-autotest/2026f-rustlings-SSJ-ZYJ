// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
// 这是关于以下章节的小测验：
// - 变量
// - 函数
// - 条件表达式

// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given
// the quantity bought. No hints this time!
//
// 玛丽正在购买苹果。苹果的价格计算方式如下：
// - 一个苹果售价 2 rustbucks。
// - 如果玛丽购买超过 40 个苹果，每个苹果只需 1 rustbuck！
// 编写一个函数，根据购买数量计算订单价格。不提供提示！

// Put your function here!
// fn calculate_price_of_apples {
// 在这里编写你的函数！
fn calculate_price_of_apples(n: i32) -> i32 {
    if n <= 40 {
        2 * n
    } else {
        n
    }
}
// Don't modify this function!
// 不要修改这个函数！
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
