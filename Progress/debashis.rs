// Rust variables ===>

// ########################
// ### Date: 10/12/2023 ###
// ########################

// Initialize using let command, can declare type, by default immuatble.'
// Example: 
let mut x: i32 = 6; // make it mutable using mut cmd

// If we try to modify immuatable variables, we expect an error.

// ***************************************************************
// variable shadowing ==>

fn main() {
    let x: i32 = 11;
    {
        let x = 7;
        println!("{}", x); // Outputs 7
    }
    println!("{}", x); // Prints 11
    let x = 58;
    println!("{}", x); // Prints 58

}

// variable destructuring ===>
let (x, y) = (55, 3);
let mut x = x;

assert_eq!(x, 55);

// Tuples ====>

let ageAndGender = (27, "M");

println!("{}", ageAndGender.0); // Prints 27

println!("{}", ageAndGender.1); // Prints M