use std::io;

struct User {
    id: usize,
    username: String,
    password: String,
    message: String,
}

fn main() {
    let mut input = String::new();

    let mut users: Vec<User> = vec![];

    loop {
        println!("Welcome to the allmost secure message keeper, do you want to log in? [y/n]");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => continue,
        };

        match input.trim() {
            "y" => {
                if (users.len() == 0) {
                    println!(
                        "It seems like there are no users, do you want to create a new account? [y/n]"
                    );
                    input.clear();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {}
                        Err(_) => continue,
                    };
                    match input.trim() {
                        "y" => {
                            println!("Please input a username:");

                            let mut username = String::new();

                            match io::stdin().read_line(&mut username) {
                                Ok(_) => {}
                                Err(_) => continue,
                            };
                            let username = username.trim();

                            println!("Please input a password for {}:", username);
                            input.clear();

                            let mut password = String::new();

                            match io::stdin().read_line(&mut password) {
                                Ok(_) => {}
                                Err(_) => continue,
                            };
                            let password = password.trim();

                            let user_id: usize = users.len();

                            users.push(User {
                                id: users.len(),
                                username: String::from(username),
                                password: String::from(password),
                                message: String::new(),
                            });

                            println!(
                                "Congrats, your account with the name {} has the id {}",
                                users[user_id].username, user_id
                            )
                        }
                        "n" => continue,
                        _ => {
                            println!("Invalid option {input}")
                        }
                    }
                } else {
                    println!("Do you want to (1) log in or (2) create a new account?")
                }
            }
            "n" => {
                println!("Do you want to exit?");
                input.clear();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {}
                    Err(_) => continue,
                };
                match input.trim() {
                    "y" => break,
                    "n" => continue,
                    _ => continue,
                }
            }
            _ => {
                println!("Invalid option {input}")
            }
        }

        input.clear()
    }
}
