use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        // 多分これはread_lineでpythonのinput("")みたいなことができて第一引数で、指定先のやつをやっているのかなって思った。
        .read_line(&mut input)
        .expect("読み込みに失敗しました。");
    println!("{}", input);
    println!("Hello, world!");
}
