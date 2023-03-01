fn main() {

    // Rust has variables, mutable variables and constants.
    
    //Variables are assigned using the 'let' keyword".
    let num = 10;
    let mut mut_num = 15;

    // Variables can infer what datatype to assign, but you can also explicitly declare them.
    let my_int: i32 = 768;
    let mut my_float: f64 = 87834.034;

    //Constants are values that can never be changed. 
    //They are useful for values that you want to hardcode. They are also useful for variables you want to be visible globally.
    const HIGH_SCORE: i8 = 30;

    //Data Types

    //Rust is a statically typed language - it must have a type assigned to each value.
    // Types are subdivided into Scalar and Composite.

    //Scalar types are single value objects. 
    // These include Integers, Floating-Point values, Booleans and Character
    //Integers in Rust are also subdivided.

    //Signed or Unsigned 
    //Signed integers are able to be positive or negative numbers.
    let negative_number: i16 = -87;
    //Unsigned integers are assumed to be positive.
    let positive_number: u128 = 983429347234;

    //Integers are also subdivided by their size.
    //You can specify whether an integer is 8 bit, 16, 32, 64 or 128.

    let small_number: u8 = 9;
    let big_number: u128 = 983492834729834729347;

    //Integer overflow.
    //If you have an integer in a build, and you try and assign a number outside it's accepted size, 2 things can happen:
        //IF in debug mode, Rust will 'panic' and exit with an error code.
        //If in a production build, the program will 'wraparound'. This means for an 8bit integer, if you tried to assign a value of 256 (1 over the max of 255), 
        //will instead give the value 0. This is a bad thing to have happen. 


    //Floats
    //Rust has 2 primitive floating point numbers - 32 bit and 64 bit.
    let float_1: f32  = 0.973;
    let float_2: f64 = 298347298347.92834293847;

    //Booleans
    //booleans are assigned using the bool word.
    let t: bool = true;
    //or just assigned implicitly.
    let f = false;

    //Characters
    //Rust characters are values storing a single unicode value.
    //Therefore they can be used for alphanumerics, Japanese, Chinese, Korean, accented or even emoji values.
    //Characters are assigned with the 'char' keyword.
    let my_new_char: char = 'b';
    //Important - note the use of single quotes - double quotes are used for Strings.


    //Composite values.
    //There are 2 primitive composite values - Tuples and Lists.

    //Tuples
    //These composite values form a fixed number of elements, that can have different types.
    //They are formed with comma separated values within a pair of parentheses.

    let my_new_tuple: (i32, f64, char) = (34,9.8, 'a');

    //There are a couple of ways to access elements of a tuple.
    //One method is to 'destructure' it.

    let (x, y, z) = my_new_tuple;
    //This assigns the values implicitly from the tuple to 3 independent variables.

    //Another method is to access it using a period and number:

    let accessing_tuple_example: i32 = my_new_tuple.0;

    //Arrays
    //Arrays are fixed length collections that must have all values be the same datatype.
    //This is different to other languages, where arrays are able to change size.
    //Later on we will see a vector, which is able to grow and shrink.

    //arrays are assigned in square bracketed, comma-separated values.
    let my_array: [i32; 5] = [1,2,3,4,5];

    //You can also assign all values the same using this format:
    let my_2nd_array: [i32;5] = [3;5]; 
    //this tells it to make an array, with length of 5, and all values 3.

    //Accessing an array element.
    //You can access an array element with: name[index]

    let accessing_an_element = my_2nd_array[2];
    //If you try accessing an element outside the length, Rust returns an error.
    


}
