// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // 用双引号包裹的比如"hello"在rust中被称为&str, 不是String类型,
    // 但是&str可以通过调用.to_string()方法转换成String类型
    "blue".to_string()
}
