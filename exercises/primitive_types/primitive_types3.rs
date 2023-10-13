// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // define the array way 1  let array_name = [v1, v2, ...];
    let a1 = [1, 2, 3, 4, 5, 5];
    println!("the array a is {:?}", a1);

    // define the array way 2 let array_name : [data_type;elements_number] = [v1, v2, ...];
    // [data_type;elements_number] 中间的符号是 ';'
    let a2: [u8; 3] = [1, 2, 3];
    println!("the array a is {:?}", a2);

    // define the array way 3  let array_name = [initail_val;elements_number];
    // The array named a will contain 3 elements that will all be set to the value "Go!" initially.
    let a = ["Rust!";103];
    println!("the array a is {:?}", a);

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
