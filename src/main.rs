use std::io;

fn main() {
    println!("Hello, world!");
    
    let a = 5;  // Immutable variable
    let mut b = 10;  // Mutable variable
    b = 12;  // Reassigning the mutable variable
    
    println!("Value of a: {}", a);
    println!("Value of b: {}", b);
    
}

