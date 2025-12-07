//Rust program to calculate total cost

struct laptop {
    brand: String,
    price: u32,
}

impl laptop{
    // Method to calculate total price for a given quantity
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // Create laptop items
    let hp = laptop {
        brand: String::from("HP"),
        price: 650_000,
    };

    let ibm = laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };

    let toshiba = laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };

    let dell = laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    // Quantity customer buys from each brand
    let qty = 3;

    // Compute total cost
    let total = hp.total_cost(qty) + ibm.total_cost(qty) + toshiba.total_cost(qty) + dell.total_cost(qty);

    // Display results
    println!("Total cost for buying 3 laptops from each brand is: ₦{}", total); 


    println!("Hello, world!");
}
