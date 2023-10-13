// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    // 在rust中以分号;结尾的被称为语句, 不能当作类型进行返回, 表达式可以作为返回值返回
    // 因为表达式最终都会有个一结果, 这个结果一定有对应的类型, 如果
    // 表达式的结果与函数的返回值类型一致, 就可以直接返回对应的表达式即可
    // num * num;
    num * num
}
