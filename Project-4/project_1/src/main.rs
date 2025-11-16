// Rust program to calculate the area or volume of different shapes

use std::f64::consts::PI;
use std::io;

// function to calculate the area of trapezium, rhombus, paralleleogram, cube
fn trapezium_area(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn parallelogram_area(base: f64, altitude: f64) -> f64{
    base * altitude
}

fn cube_area(length: f64) -> f64 {
    6.0 * length * length
}

// function to calculate the volume of cylinder
fn cylinder_volume(radius: f64, height: f64) -> f64 {
    PI * radius * radius * height
}

fn main() {
    println!("MTH 101 Shape Calculator");
    println!("Select a shape to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    println!("6. Exit");

    println!("\nEnter your choice (1-6):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Invalid number");
        

    if choice == 1 {
        println!("Calculating Area of Trapezium");
        println!("Formula: height / 2.0 * (base1 + base2)");

        let height = get_input("Enter height: ");
        let base1 = get_input("Enter base1: ");
        let base2 = get_input("Enter base2: ");

        let area = trapezium_area(height, base1, base2);
        println!("Area of Trapezium: {:.2}", area);
    }
    else if choice == 2 {
        println!("Calculating Area of Rhombus");
        println!("Formula: 0.5 * diagonal1 * diagonal2");

        let diagonal1 = get_input("Enter diagonal1: ");
        let diagonal2 = get_input("Enter diagonal2: ");

        let area = rhombus_area(diagonal1, diagonal2);
        println!("Area of Rhombus: {:.2}", area);
    }
    else if choice == 3 {
        println!("Calculating Area of Parallelogram");
        println!("Formula: base * altitude");

        let base = get_input("Enter base: ");
        let altitude = get_input("Enter altitude: ");

        let area = parallelogram_area(base, altitude);
        println!("Area of Parallelogram: {:.2}", area);
    }
    else if choice == 4 {
        println!("Calculating Area of cube");
        println!("Formula: 6.0 * (length)^2");

        let length = get_input("Enter side lenth: ");
           
        let area = cube_area(length);
        println!("Surface Area of cube: {:.2}", area);
    }
    else if choice == 5 {
        println!("Calculating Volume of Cylinder");
        println!("Formula: π * (radius)^2 * height");

        let radius = get_input("Enter radius: ");
        let height = get_input("Enter height: ");

        let volume = cylinder_volume(radius, height);
        println!("Volume of Cylinder: {:.2}", volume);
    }
    else if choice == 6 {
        println!("Thank you for using the Shape Calculator");
    }
    else {
        println!("Invalid choice! Please enter a valid number between 1 and 6.");
    }
    
}    

// Helper function to get numeric input from user
fn get_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input_trimmed = input.trim();
        if input_trimmed.parse::<f64>().is_ok() {
            return input_trimmed.parse().unwrap();
        } else {
            println!("Please enter a valid number!");
        }
    }
}