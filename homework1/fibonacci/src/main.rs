use std::io;

fn main() {
    let mut input_number = String::new();

    println!("Please input your number:");

    io::stdin()
        .read_line(&mut input_number)
        .expect("Error reading the input");

    let input_number: i32 = match input_number.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("The input value was not a valid number");
            return;
        }
    };

    let mut x: i32 = 0;
    let mut y: i32 = 1;
    let mut z: i32;

    let output = loop {
        z = x + y;
        x = y;
        y = z;

        if z > input_number {
            break x;
        }
    };

    println!("The last fibonacci number, smaller than the input is: {output}")
}
