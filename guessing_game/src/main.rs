use std::io;
use rand::Rng;
use std::cmp::Ordering;
//The line above tells Rust to import the input-output modules from the standard library.

//main function is the function called by the program initially.
fn main() {
    guessing_game_04();
}

fn guessing_game_01() {
    println!("Guess the number!");
    //println! is a macro.
    println!("Please input your guess.");

    //This line creates a mutable variable of the String type.
    //String::new() is a function that creates a new instance of a string object.
    // :: syntax shows an associated function. This indicates that it is a function implemented on a type.
    // ::new() is a very common function used by many types. 
    let mut guess = String::new();

    //This segment uses the input module.
    //io::stdin() returns an instance of a Stdin object, which is used to handle input from the terminal.
    //.read_line(&mut guess) calls the read_line() method. 
    //the argument '&mut guess' tells the function where to store the string input. 
    // the ampersand '&' indicates that the argument is a reference. We will discuss them in chapter 4. 
    //References need to have mut specified, else they are immutable by default, like variables. 
    //note that the function is broken into multiple lines for improved readability. 
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        //.expect is used 
        //read_line() actually turns whatever was input into a Result value, behind the scenes, which is an enumeration.
        //Result types are used because they allow us to encode error handling information. The 2 enum states are 'Ok' and 'Err'
        //.expect() is a method of the Result type. It shows the message that should be displayed if the Result enum is 'Err'

    println!("You guessed {guess}");
    //The curly brackets here are a placeholder. You can put a variable in the curly brackets. 
    //Another option is to have a 'format string'. E.g:
    let y = 10;
    println!("y + 2 = {}", y + 2);
    //This works in a comma-separated list. Each curly bracket is associated with a value sequentially.
}

//Part 2 uses a Crate to generate a secret number for us to guess. We'll use a number between 1 and 100.
//For this we will use a library crate. This is what Cargo is great at. 
//In the Cargo.toml file, add rand to the dependencies. 
//When we build this project now, Cargo retrieves the rand crate from Crates.io, along with any Crates that rand itself has as a dependency.
//When a dependency is added, Cargo also automatically adds it to the Cargo.lock file. This way, the same dependency can be used at a later date. 

fn guessing_game_02() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
                              .gen_range(1..=100);

    println!("The random number is {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

//Part 3 adds comparison between our secret number and our input.
//We add another library - use std::cmp::Ordering
fn guessing_game_03(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
                              .gen_range(1..=100);

    
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //We need to convert guess from a String to an integer. 
    //Here we are shadowing guess with an unsigned integer. 
    //If the expression fails to parse, then we display the message "Please type a number".
    let guess: u32 = guess.trim()
                          .parse()
                          .expect("Please type a number");

    println!("You guessed: {guess}");

    //Ordering is an enum. It can have values of Less, Greater or Equal. 
    //cmp compares guess to the secret number.
    //match expression tells what behaviour to do, depending on the value of Ordering. 
    //Match statements have arms. Each arm is a pattern to match against. They will be covered later in more detail.

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    
    }
}

//We now need to add looping
fn guessing_game_04() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng()
                              .gen_range(1..=100);

    //We add a loop to repeat this code until a break is called. 

    loop {
        println!("Please input your guess.");


        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // This code has been changed from reporting an error, to handling one.
        // parse method returns a Result instance.
        // We use a match expression to change our behaviour, depending on the Result value.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}