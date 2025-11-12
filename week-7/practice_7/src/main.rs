fn main() {
    // Array with data type (explicit integer datatype)
    let arr1 = [10, 20, 30, 40];
    println!("arr1 with data type");
    println!("Array is: {:#?}", arr1);
    println!("Array size is {}", arr1.len());

    // Array with data type (explicit float datatype)
    let arr2 = [10.4, 20.4, 30.5, 51.2];
    println!("arr2 with data type (float)");
    println!("Array is: {:#?}", arr2);
    println!("Array size is {}", arr2.len());

    // Array with default values that creates and
    // initializes an array of size 8 with a default value of -1.
    let arr3 = [-1; 8];
    println!("arr3 with default values");
    println!("Array is: {:#?}", arr3);
    println!("Array size is {}", arr3.len());
}