// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.
// 这与 from_into.rs 类似，但这次我们将实现 `FromStr`，并在出错时返回错误，而不是回退到默认值。
// 实现 FromStr 后，还可以对字符串使用 `parse` 方法来生成实现该 trait 的对象。
// 更多信息请参阅 https://doc.rust-lang.org/std/str/trait.FromStr.html。

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
// 我们将使用此错误类型实现 `FromStr`。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    // 空的输入字符串。
    Empty,
    // Incorrect number of fields
    // 字段数量不正确。
    BadLen,
    // Empty name field
    // 姓名字段为空。
    NoName,
    // Wrapped error from parse::<usize>()
    // parse::<usize>() 返回的错误
    ParseInt(ParseIntError),
}

// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.
// 步骤：
// 1. 如果输入字符串长度为 0，应返回错误。
// 2. 按其中的逗号分割字符串。
// 3. 分割结果必须只有 2 个元素，否则返回错误。
// 4. 取分割结果的第一个元素作为姓名。
// 5. 取另一个元素，并使用类似 `"4".parse::<usize>()` 的方式将其解析为 `usize` 类型的年龄。
// 6. 如果提取姓名或年龄时出现问题，应返回错误。
// 如果一切顺利，则返回包含 Person 对象的 Result。
//
// 补充说明：`Box<dyn Error>` 实现了 `From<&'_ str>`。这意味着如果想返回字符串错误信息，
// 可以直接使用 `Err("my error message".into())`。

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        let name = parts[0];
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        let age = parts[1]
            .parse::<usize>()
            .map_err(ParsePersonError::ParseInt)?;

        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
