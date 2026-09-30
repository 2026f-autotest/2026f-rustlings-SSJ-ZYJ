// primitive_types2.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)
// 完成缺失代码所在的那一行！没有提示，也没有陷阱，只是熟悉一下这些写法 :)

fn main() {
    // Characters (`char`)
    // 字符（`char`）

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    // 注意使用单引号，它们不同于前面见过的双引号。
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = '6';
    // Finish this line like the example! What's your favorite character?
    // 像示例一样完成这一行！你最喜欢的字符是什么？
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    // 可以尝试字母、数字、特殊字符、其他语言中的字符，甚至 emoji！
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
