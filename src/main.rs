// lets Rust know we'll be using "rand" as external dependency
extern crate rand;

// this project is a binary crate, which is an excutable
// The "rand" crate is a library crate, which contains code intended to be used in other programs
// bring io(input/output) library into scope
// io library comes from standard library(std)
use std::io;
// "Ordering" is another enum, like "Result", but the variants for Ordering are "Less", "Greater", and "Equal"
use std::cmp::Ordering;
// "Rng" is a trait that defines methods that random number generators implement
// this trait must be in scope for us to use those methods
use rand::Rng;

fn main() {
    // println! is a macro that prints a string to screen
    println!("Guess the number!");

    // rand::thread_rng function give us the particular random number generator to use
    // one that is local to the current thread of execution and seeded by the operating system
    // gen_range method is defined by the Rng trait we brought into the scope
    // gen_range method takes two numbers as arguments and generates a random number between them
    // inclusive of lower bound by exclusive on the upper bound
    let secret_number = rand::thread_rng().gen_range(1,101);

    // println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // loop gives us an infinite loop
    loop{
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

        // shadows the previous value of "guess" with a new one
        // shadowing is often used to convert a value from one type to another type
        // shadowing lets use reuse the guess variable rather than forcing use to create two uniq variables
        // "trim" method on "String" instance will eliminate any whitespace at beinning and end
        // u32 can only contain numerical characters
        // user must press Return key to satisfy read_line
        // when user presses Return, a newline character is added to the string
        // eg. user types "5" and presses return, "guess" looks like this "5\n"
        //  "trim" method removes "\n" leaving just 5
        // parse method on string parses string into some kind of number
        // need to tell parse exact type we want by using let guess: u32
        // the colon(:) after guess tells Rust we'll annotate the variable's type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            // "parse" method could easily cause an error
            // "parse" method returns a "Result"
            // if "parse" returns an "Err" "Result" because it couldn't create anumber from the string, the expect call will crash the game and print the messag we give it
            // if "parse" returns an "Ok", "expect" will return the number that we want from the "Ok" value
            // .expect("Please type a number!");

        // "{}" is a place holder that holds the value in place
        // "{}" here holds the first value listed after the format string
        // let x = 5;
        // let y = 10;
        // println!("x = {} and y = {}", x, y) ==> x = 5 and y = 10
        println!("You guess: {}", guess);

        // "cmp" method compares twoo values and can be called on anything that can be compared
        // "cmp" returns a variant of Ordering
        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            // program exits the loop once user guesses correctly
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }
    }
}
