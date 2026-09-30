// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint modules1` 获取提示，或使用 watch 子命令中的 hint。

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    // 不要让模块外部的任何人看到这个函数！
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
