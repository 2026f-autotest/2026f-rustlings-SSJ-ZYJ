// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.
// From trait 用于值到值的转换。如果正确实现了 From，Into trait 就应当可以反向工作。
// 更多信息请参阅 https://doc.rust-lang.org/std/convert/trait.From.html。
//
// 执行 `rustlings hint from_into` 获取提示，或使用 watch 子命令中的 hint。

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
// 我们实现 Default trait，将其作为输入字符串无法转换为 Person 对象时的后备方案。
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results
// 你的任务是完成此实现，使 `let p = Person::from("Mark,20")` 能够编译。
// 注意：需要使用类似 `"4".parse::<usize>()` 的方式将年龄部分解析为 `usize`，并正确处理解析结果。
//
// 步骤：
// 1. 如果输入字符串长度为 0，返回 Person 的默认值。
// 2. 按其中的逗号分割字符串。
// 3. 取分割结果的第一个元素作为姓名。
// 4. 如果姓名为空，返回 Person 的默认值。
// 5. 取分割结果的另一个元素，并将其解析为 `usize` 类型的年龄。
// 如果解析年龄时出现问题，返回 Person 的默认值；否则返回一个包含这些结果的 Person 对象。

// I AM NOT DONE

impl From<&str> for Person {
    fn from(s: &str) -> Person {
    }
}

fn main() {
    // Use the `from` function
    // 使用 `from` 函数。
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    // 由于已经为 Person 实现了 From，因此应该可以使用 Into。
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        // 测试默认 Person 是否是 30 岁的 John。
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        // 测试传入错误字符串时是否返回 John。
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        // 测试 `"Mark,20"` 是否有效。
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        // 测试由于年龄解析错误，`"Mark,twenty"` 是否返回默认 Person。
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
