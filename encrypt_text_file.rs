use std::fs;
use std::io::{self, Read, Write};

fn encrypt(text: &str) -> String {
    text.chars()
        .map(|c| (c as u8).wrapping_add(1) as char)
        .collect()
}

fn decrypt(encrypted_text: &str) -> String {
    encrypted_text
        .chars()
        .map(|c| (c as u8).wrapping_sub(1) as char)
        .collect()
}

fn main() {
    loop {
        println!("Menu:");
        println!("1. Encrypt File");
        println!("2. Decrypt File");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter the path of the file to encrypt:");
                let mut file_path = String::new();
                io::stdin()
                    .read_line(&mut file_path)
                    .expect("Failed to read line");

                let file_path = file_path.trim();
                let original_text = fs::read_to_string(&file_path).unwrap_or_else(|_| String::new());
                let encrypted_text = encrypt(&original_text);

                let encrypted_file_path = format!("{}_encrypted.txt", file_path);
                let mut encrypted_file = fs::File::create(&encrypted_file_path).expect("Failed to create file");
                encrypted_file.write_all(encrypted_text.as_bytes()).expect("Failed to write to file");

                println!("File encrypted and saved to: {}", encrypted_file_path);
            }
            "2" => {
                println!("Enter the path of the file to decrypt:");
                let mut file_path = String::new();
                io::stdin()
                    .read_line(&mut file_path)
                    .expect("Failed to read line");

                let file_path = file_path.trim();
                let encrypted_text = fs::read_to_string(&file_path).unwrap_or_else(|_| String::new());
                let decrypted_text = decrypt(&encrypted_text);

                let decrypted_file_path = format!("{}_decrypted.txt", file_path);
                let mut decrypted_file = fs::File::create(&decrypted_file_path).expect("Failed to create file");
                decrypted_file.write_all(decrypted_text.as_bytes()).expect("Failed to write to file");

                println!("File decrypted and saved to: {}", decrypted_file_path);
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
