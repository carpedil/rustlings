// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // 2. 但是此处声明的word是一个字符串String类型, 且此处不允许修改word的类型
    let word = String::from("green"); // Try not changing this line :)
    // 3. 所以在调用is_a_color_word函数时, 可以在word前面添加&符号即可
    // 为什么? 待研究TODO
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

// 1. is_a_color_word 函数要求传入的attempt是一个&str类型
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
