use std::io::Write;

fn main() {
    // Create the content for the file
    let content = "Drink Categories

    Lager:
    33 Export
    Desperados
    Goldberg
    Gulder
    Heineken
    Star

    Stout:
    Legend
    Turbo King
    Williams

    Non-Alcoholic:
    Maltina
    Amstel Malta
    Malta Gold 
    Fayrouz
    ";

    let mut file = std::fs::File::create("Nigeria breweries Drinks.txt").expect("create failed");
    file.write_all("Nigeria Breweries Plc\n"
        .as_bytes()).expect("write failed");
    file.write_all(content.as_bytes()).expect("write failed");
    println!("\nData written to file.");

}
