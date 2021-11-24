mod module1;
mod module2;
mod module3;
include!("module4/function_four.rs");

fn main() {
    module1::function_one();
    module2::function_two();
    module3::function_three();
    function_four();
}
