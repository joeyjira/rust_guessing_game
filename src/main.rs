// bring io(input/output) library into scope
// io library comes from standard library(std)
use std::io

fn main() {
    // println! is a macro that prints a string to screen
    println!("Guess the number!");

    println!("Please input your guess.");

    // "let" is used to create variables
    // in Rust, variables are immutable by default
    // mut makes a variable mutable
    // binds "guess" to a new instance of "String"
    // "::" indicates new is an associated function of "String" type
    // an associated function is implemented on a type rather than on a particular instance of "String"
    // some langues call this static method
    let mut guess = String::new();

    // could have called std::io::stdin() instead
    // .read_line(&mut guess) is method on standard input to get input from user
    // read_line takes whatever user types into standard input and place that into a string that was passed as argument
    // string needs to be mutable so the method can change the string's content
    // & indicates that this argument is a reference
    // references lets multiple parts of you code access one piece of data without needing to copy that data into memory multiple times
    // read_line returns a value of io::Result
    // Rust has a number of types named Result in its standard library
    // generic "Result" as well as specific versions for submodules, such as io::Result
    // Result types are enumerations(enums)
    // Enumeration is a type that can have a fixed set of values which are called the enum's variants
    // Result has variants of "Ok" or "Err"
    // "Ok" indicates that operation was successful and inside the "Ok" variant is the successfully generated value
    // "Err" means the operation failed, and contains information on why it failed
    // purpose of Result types is to encode error handling information
    io::stdin().read_line(&mut guess)
        // Result types have methods defined on them
        // instance of io::Result has expect method
        // if instance of io::Result is an "Err" value, expect will cause the program to crash and display the message that you passed as argument to expect
        // if "expect" not called then the program will compile but we'll get a warning
        .expect("Failed to read line");
    
    // "{}" is a place holder that holds the value in place
    // "{}" here holds the first value listed after the format string
    // let x = 5;
    // let y = 10;
    // println!("x = {} and y = {}", x, y) ==> x = 5 and y = 10
    println!("You guess: {}", guess);
}
