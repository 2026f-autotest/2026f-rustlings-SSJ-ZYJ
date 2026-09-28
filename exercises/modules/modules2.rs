// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
// 可以使用 `use` 和 `as` 关键字将模块路径引入作用域并为其提供新名称。
// 修复这些 `use` 语句，使代码通过编译。
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint modules2` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

mod delicious_snacks {
    // TODO: Fix these use statements
    // TODO：修复这些 use 语句。
    use self::fruits::PEAR as ???
    use self::veggies::CUCUMBER as ???

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
