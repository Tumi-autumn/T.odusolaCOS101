fn main() {
    // while true loop using 'loop'
    let mut x = 0;

    loop {
        x += 1;
        println!("x = {}", x);

        if x == 15 {
            break;
        }
    }
}