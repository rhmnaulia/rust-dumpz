use std::collections::HashMap;
use std::io;

fn generate_short_url() -> String {
    // Generate a random short URL for simplicity (not secure in real-world scenarios) this only for sake of my learning
    let random_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let short_url: String = (0..6)
        .map(|_| *random_chars.choose(&mut rand::thread_rng()).unwrap())
        .collect();
    short_url
}

fn main() {
    let mut url_map: HashMap<String, String> = HashMap::new();

    loop {
        println!("Menu:");
        println!("1. Shorten URL");
        println!("2. Access Shortened URL");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter the long URL to shorten:");
                let mut long_url = String::new();
                io::stdin()
                    .read_line(&mut long_url)
                    .expect("Failed to read line");

                let short_url = generate_short_url();
                url_map.insert(short_url.clone(), long_url.trim().to_string());

                println!("Shortened URL: {}", short_url);
            }
            "2" => {
                println!("Enter the short URL to access the original URL:");
                let mut short_url = String::new();
                io::stdin()
                    .read_line(&mut short_url)
                    .expect("Failed to read line");

                match url_map.get(&short_url.trim().to_string()) {
                    Some(original_url) => {
                        println!("Original URL: {}", original_url);
                    }
                    None => {
                        println!("Short URL not found");
                    }
                }
            }
            "3" => {
                println!("Exiting. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 3.");
            }
        }
    }
}
