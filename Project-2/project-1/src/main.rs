//Rust program to find the roots of a quadratic equation
//and the discriminant

use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    //Input values for a, b, and c
    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    //Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    //Determine and print the nature of the roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots:");
        println!("Root1 = {}", root1);
        println!("Root2 = {}", root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("Exactly one real roots");
        println!("Root = {}", root)
    } else{
        println!("The roots are complex and imaginary (no real roots).");
    }

}
