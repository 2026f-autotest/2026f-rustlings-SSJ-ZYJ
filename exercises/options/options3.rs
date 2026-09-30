// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint options3` 获取提示，或使用 watch 子命令中的 hint。

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
       // 修复此处，但不要删除这一行。
}
