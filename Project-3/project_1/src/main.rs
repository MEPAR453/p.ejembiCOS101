// Rust program to display menu for the food items available and take order from the customer
use std::io;
fn main() {

    println!("Welcome to Mimi's foodstore");

    println!("Menu");
    println!("P = Poundo yam/Edinkaiko Soup   -  3_200.0");
    println!("F = Fried Rice & Chicken        -  3_000.0");
    println!("A = Amala & Ewedu Soup          -  2_500.0");
    println!("E = Eba & Egusi Soup            -  2_000.0");
    println!("W = White Rice & Stew           -  2_500.0");

    let mut food_type = String::new();
    println!("Enter the food type (P, F, A, E, W): ");
    io::stdin().read_line(&mut food_type).expect("failed to read input");
    let food_type = food_type.trim().to_uppercase();

    let mut quantity = String::new();
    println!("Enter quantity: ");
    io::stdin().read_line(&mut quantity).expect("failed to read input");
    let quantity:f32 = quantity.trim().parse().expect("Invalid number entered");

    //Determine price   
    let price_per_item:f32; 

    if food_type ==  "P" {
       price_per_item = 3200.0;
    } else if food_type =="F" {
       price_per_item = 3000.0;
    } else if food_type ==  "A" {
       price_per_item = 2500.0;
    } else if food_type == "E" {
       price_per_item = 2000.0;
    } else if food_type == "W" {
       price_per_item = 2500.0;
    } else {
        println!("Invalid food type entered!");
        return;
    }

    //Calculate total
    let mut total = price_per_item * quantity;

    //Apply discount if total > 10,000
    if total > 10000.0 {
        let discount = 0.05 * total;
        total -= discount;
        println!("A 5% discount has been applied. Discount: ₦{:.2}", discount);
    }

    //output results
    println!("Price per item: ₦{:.2}", price_per_item);
    println!("Quantity: {}", quantity);
    println!("Total amount to pay: ₦{:.2}", total);


    println!("Thank you for your order!");
}   



