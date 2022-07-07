use std::io;

fn main() {
    loop {
        let mut input = String::new();

        println!("Please input your *C temp to be converted:");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => continue,
        }

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let fahrenheit = input * 9 / 5 + 32;

        println!("{input} *C = {fahrenheit} *F");
    }
}
