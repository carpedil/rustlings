// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // rust可以自动根据函数call_me的形参进行自动推断变量的类型
    call_me(3);
    // 等价于cell_me(8)
    call_me(8u32);
    // 类型就不匹配, 调用会报错
    // let n = 8i8;
    // call_me(n);
}

fn call_me(num:u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
