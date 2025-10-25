use std::io;
use std::f64;

fn main() {
    // Prompt user for values of a, b, and c
    println!("This program finds the roots of a quadratic equation (ax² + bx + c = 0)");

    // Input a
    println!("Enter value of a:");
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("Failed to read input");
    let a: f64 = a_str.trim().parse().expect("Please enter a valid number");

    // Input b
    println!("Enter value of b:");
    let mut b_str = String::new();
    io::stdin().read_line(&mut b_str).expect("Failed to read input");
    let b: f64 = b_str.trim().parse().expect("Please enter a valid number");

    // Input c
    println!("Enter value of c:");
    let mut c_str = String::new();
    io::stdin().read_line(&mut c_str).expect("Failed to read input");
    let c: f64 = c_str.trim().parse().expect("Please enter a valid number");

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    println!("\nThe discriminant is: {}", discriminant);

    // Determine the nature of the roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The equation has two distinct real roots:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The equation has one real root:");
        println!("Root = {}", root);
    } else {
        println!("The equation has no real roots.");
    }
}