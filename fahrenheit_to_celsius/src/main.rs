use std::io;

fn main() {
//Practice program from chapter 3 of the Rust lang book.
//Used a loop to keep running the program if a user doesn't enter a valid number.
//Used a match statement to change behaviour depending on the value of the 'parse' method.
//Parse returns a Result instance, which can either be ok or err.
//Finally, I performed the mathematical operation, then used String formatting to specify that I only wanted 2 decimal points.
    
    'primary_loop:loop { 
        println!("Please enter your temperature:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Not a number");
        
        let input: f64=match input.trim().parse::<f64>() {
            Err(_) => {
                println!("That is not a number - please try again");
                continue;
                },
            Ok(num) => num,                
        };
        
        println!("Your input is {input}.");
        let c: f64 = 5.0/9.0 * (input - 32.0);
        println!("That converts to {:.2} degrees c.",c);
        break 'primary_loop;
    }; 
}

// C = 5/9 x (F-32) 