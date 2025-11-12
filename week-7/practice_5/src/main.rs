fn main() {
    let num: i32 = 5;
    // When passed to the function, the value of 'num_to_zero' is copied.
    let num_to_zero = num; 
    
    // Call the function
    mutate_num_to_zero(num_to_zero);

    // The original variables remain 5
    println!("The value of num is: {}", num);
    println!("The value of num_to_zero is: {}", num_to_zero); 
}

// Function receives a copy of the value
fn mutate_num_to_zero(mut param_num: i32) {
    param_num = 0; // Only the local copy is changed
    println!("param_num value is: {}", param_num); // Prints 0
}