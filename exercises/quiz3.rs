// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
// 本小测验考查以下章节：
// - 泛型
// - trait
//
// 一个虚构的魔法学校用 Rust 编写了新的成绩单生成系统！
// 当前系统只支持生成数字成绩的成绩单（例如 1.0 -> 5.5）。
// 但学校还会使用字母成绩（A+ -> F-），因此需要同时打印这两种成绩单！
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
// 修改 `ReportCard` 结构体和 impl 块中的必要代码，以支持字母成绩。
// 将第二个测试中的成绩改为 "A+"，以证明修改支持字母成绩。
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.
// 执行 `rustlings hint quiz3` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        // TODO：完成题目后，记得将这里的成绩改为字母成绩。
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
