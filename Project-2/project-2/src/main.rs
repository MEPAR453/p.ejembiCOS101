// Rust program taking the experience and age of an employee's input
//to determine annual incentive

use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
   
    //input experience and age 
    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let experience = input1.trim().to_lowercase();

    println!("Enter the age of the employee: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age: u32 = input2.trim().parse().expect("Please enter a valid number!");

    // Determine incentive
    let incentive: u32;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age < 40 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000; 
        } else {
            //For experienced employees not covered by the above conditions
            incentive = 1_000_000
        }

    } else {
        incentive = 100_000;
    }

    println!("The employee's annual incentive is: N{}", incentive);
    
}
