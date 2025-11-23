// Rust program using vectors + tuples to find developer with highest programming experience.

use std::io;

fn main() {
    // Vector of tuples: Name, Years of Experience)
    let mut applicants: Vec<(String, u32)> = Vec::new();

    let mut number = String::new();
    println!("How many applicants are being interviewed?");
    io::stdin().read_line(&mut number).unwrap();
    let number: u32 = number.trim().parse().unwrap_or(0);

    // Collect data for each applicant
    for i in 1..=number {
        println!("\nEnter name of applicant {}:", i);
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();

        println!("\nEnter year of programming experience for {}:", name.trim());
        let mut years = String::new();
        io::stdin().read_line(&mut years).unwrap();
        let years: u32 = years.trim().parse().unwrap_or(0);

        applicants.push((name.trim().to_string(), years));
    }

    // Find applicants with highest experience
    let mut top_applicant = ("None".to_string(), 0);

    for (name, exp) in applicants {
        if exp > top_applicant.1 {
            top_applicant = (name, exp);
        }
    }

    // Output result
    println!("Most experienced applicant:");
    println!("Name: {}", top_applicant.0);
    println!("Years of Experience: {}", top_applicant.1);
}
