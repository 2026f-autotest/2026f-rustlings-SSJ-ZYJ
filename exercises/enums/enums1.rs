// enums1.rs
//
// No hints this time! ;)
// 本题不提供提示！;)

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    // TODO：根据下面的使用方式定义几种消息类型。
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
