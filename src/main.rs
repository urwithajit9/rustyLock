use rand::Rng;
use std::io::{self, Write};

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

const MAX_PASSWORD_LENGTH: usize = 128;
const MIN_PASSWORD_LENGTH: usize = 8;

struct PasswordOptions {
    length: usize,
    use_uppercase: bool,
    use_numbers: bool,
    use_special: bool,
}

impl Default for PasswordOptions {
    fn default() -> Self {
        Self {
            length: 12,
            use_uppercase: true,
            use_numbers: true,
            use_special: true,
        }
    }
}

fn generate_password(options: &PasswordOptions) -> String {
    let mut rng = rand::thread_rng();
    let mut password = String::with_capacity(options.length);
    let mut available_chars = String::with_capacity(
        LOWERCASE.len() + 
        if options.use_uppercase { UPPERCASE.len() } else { 0 } +
        if options.use_numbers { NUMBERS.len() } else { 0 } +
        if options.use_special { SPECIAL.len() } else { 0 }
    );
    
    // Always include lowercase letters
    available_chars.push_str(std::str::from_utf8(LOWERCASE).unwrap());
    
    if options.use_uppercase {
        available_chars.push_str(std::str::from_utf8(UPPERCASE).unwrap());
    }
    if options.use_numbers {
        available_chars.push_str(std::str::from_utf8(NUMBERS).unwrap());
    }
    if options.use_special {
        available_chars.push_str(std::str::from_utf8(SPECIAL).unwrap());
    }

    // Generate the password
    for _ in 0..options.length {
        let idx = rng.gen_range(0..available_chars.len());
        password.push(available_chars.chars().nth(idx).unwrap());
    }

    password
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_valid_length() -> usize {
    loop {
        let input = get_user_input(&format!(
            "Enter password length ({} to {}, default: 12): ",
            MIN_PASSWORD_LENGTH, MAX_PASSWORD_LENGTH
        ));
        
        if input.is_empty() {
            return 12;
        }
        
        match input.parse::<usize>() {
            Ok(length) if length >= MIN_PASSWORD_LENGTH && length <= MAX_PASSWORD_LENGTH => {
                return length;
            }
            _ => {
                println!("Please enter a number between {} and {}", MIN_PASSWORD_LENGTH, MAX_PASSWORD_LENGTH);
            }
        }
    }
}

fn main() {
    println!("Password Generator");
    println!("-----------------");

    let length = get_valid_length();

    let use_uppercase = get_user_input("Include uppercase letters? (y/n, default: y): ")
        .to_lowercase()
        .starts_with('y');

    let use_numbers = get_user_input("Include numbers? (y/n, default: y): ")
        .to_lowercase()
        .starts_with('y');

    let use_special = get_user_input("Include special characters? (y/n, default: y): ")
        .to_lowercase()
        .starts_with('y');

    let options = PasswordOptions {
        length,
        use_uppercase,
        use_numbers,
        use_special,
    };

    let password = generate_password(&options);
    println!("\nGenerated Password: {}", password);
}
