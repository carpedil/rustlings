// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.


// 1. 此处sausage_factory模块相对于当前modules1.rs而言属于其内部模块
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }
    // 3. 添加pub关键字可以让当前函数或方法在外部module中可见
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    // 2. 所以在main()函数中调用sausage_factory模块的make_sausage函数的时候, 必须确保
    // make_sausage函数对父模块/外部模块可见
    sausage_factory::make_sausage();
}
