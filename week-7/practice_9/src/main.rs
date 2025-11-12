fn main() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50]; // an array stores functions of all elements of same data type
    println!("Array is: {:?}", arr);
    let mut sum: i32 = 0;
    for val in arr.iter() {
        sum += val;
    }
    println!("Sum is: {}", sum);
}
