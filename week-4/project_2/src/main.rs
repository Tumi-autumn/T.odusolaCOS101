use std::io;

fn main() {
    println!("Employee Incentive Calculator");

    // Input experience
    println!("Is the employee experienced? (yes/no):");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    // Input age
    println!("Enter the employee's age:");
    let mut age_str = String::new();
    io::stdin().read_line(&mut age_str).expect("Failed to read input");
    let age: u32 = age_str.trim().parse().expect("Please enter a valid age");

    // Determine incentive
    let incentive: i32;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age < 40 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            // For experienced employees between 28–29 years (not clearly defined in criteria)
            incentive = 1_300_000;
        }
    } else {
        incentive = 100_000;
    }

    println!("\nThe annual incentive for this employee is ₦{}", incentive);
}