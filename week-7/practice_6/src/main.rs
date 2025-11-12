fn main() {
    // 1. Declare 'num' as mutable so its value can be changed.
    let mut num: i32 = 5; 

    println!("Original value of num: {}", num); // Prints 5
    
    // 2. Pass a mutable reference (&mut num) to the function.
    mutate_num_by_reference(&mut num);

    // 4. The original variable's value is now changed to 0.
    println!("Value of num after function call: {}", num); // Prints 0
}

// 3. Function accepts a mutable reference to an i32 (&mut i32)
fn mutate_num_by_reference(param_num: &mut i32) {
    // The dereference operator (*) is used to access and change the value 
    // at the memory address pointed to by the reference.
    *param_num = 0; 
    println!("Value inside function (changed to): {}", *param_num); // Prints 0
}