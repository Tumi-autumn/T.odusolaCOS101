fn main() {
    let city_arr: [&str; 5] = ["Abuja", "PortHarcourt", "Maiduguri", "Kano", "Lagos"];
    let mut index: usize = 0;
    while index < city_arr.len() {
        println!("Array size is {}", city_arr.len());
        index += 1;
    }
    for index in 0..city_arr.len() {
        println!("City index {} is located in {}", index, city_arr[index]);
    }
}