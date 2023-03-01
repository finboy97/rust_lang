use std::string;

fn main() {
    let x = my_name("finbar".to_string());
    println!("{x}");
}

fn another_function(my_num: i32) {
    println!("My number is {my_num}.");
}

fn my_name(input: String) -> String {
    input
}