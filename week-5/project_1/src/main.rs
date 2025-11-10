use std::io;

fn main() {
    println!("===============================");
    println!("     WELCOME TO ZEST EATERY     ");
    println!("===============================");
    println!("Menu\t\t\t\tPrice");
    println!("P = Poundo Yam/Edinkaiko Soup  - N3,200");
    println!("F = Fried Rice & Chicken       - N3,000");
    println!("A = Amala & Ewedu Soup         - N2,500");
    println!("E = Eba & Egusi Soup           - N2,000");
    println!("W = White Rice & Stew          - N2,500");
    println!("===============================");

    // Take order from user
    let mut choice = String::new();
    println!("Enter your choice (P, F, A, E, W):");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice = choice.trim().to_uppercase(); // Normalize input
    let mut price = 0;
    let mut item = "";

    match choice.as_str() {
        "P" => { item = "Poundo Yam/Edinkaiko Soup"; price = 3200; }
        "F" => { item = "Fried Rice & Chicken"; price = 3000; }
        "A" => { item = "Amala & Ewedu Soup"; price = 2500; }
        "E" => { item = "Eba & Egusi Soup"; price = 2000; }
        "W" => { item = "White Rice & Stew"; price = 2500; }
        _ => println!("Invalid selection. Please restart and enter P, F, A, E, or W."),
    }

    if price > 0 {
        println!("You ordered: {}", item);
        println!("Price: N{}", price);
        println!("Thank you for your order!");
    }
}