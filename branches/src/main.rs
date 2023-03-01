fn main() {
    let mut counter: i32 = 0;

    let result = { loop {
            if counter >= 100 {
                break counter;
            }
            counter += 1;
        }   

    };
    println!("The result is {result}");
}