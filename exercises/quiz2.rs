// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!
// 让我们通过一个函数构建一个小型机器。输入是字符串和命令列表，这些命令决定对字符串执行的操作，包括：
// - 将字符串转换为大写
// - 去除字符串两端的空白
// - 按指定次数向字符串追加 "bar"
// 具体形式如下：
// - 输入是一个由二元组组成的 Vector，第一个元素是字符串，第二个元素是命令。
// - 输出是一个字符串 Vector。
// 本题不提供提示！

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    // TODO：完成函数签名！
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        // TODO：补充输出声明！
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            // TODO：完成函数体。你可以做到！
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(n) => {
                    let mut s = string.clone();
                    for _ in 0..*n {
                        s.push_str("bar");
                    }
                    output.push(s);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // TODO：需要导入什么才能让 `transformer` 在此作用域可用？
    use super::Command;
    use crate::my_module::transformer;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
