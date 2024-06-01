use rand::{rngs::ThreadRng, Rng};
use std::time::Instant;

mod ascii;

/**
 * Return a string of letters from a..z
 */
fn get_letters() -> String {
    let alphabet: String = String::from("abcdefghijklmnopqrstuvwxyz");
    return alphabet;
}

/**
 * Run a string of symbols
 */
fn get_symbols() -> String {
    let symbols: String = String::from("!@#$%^&*_-+?");
    return symbols;
}

/**
 * Return a string of numbers from 0..9
 */
fn get_number() -> String {
    let numbers: String = String::from("0123456789");
    return numbers;
}

/**
 * Generate a secret with x length and y use of special
 * characters
 */
fn generate_secret(length: i32, use_special_chars: bool) -> String {
    // Initialize ThreadRng for rand library
    let mut rng: ThreadRng = rand::thread_rng();
    // Initialize our result variable, which this function will return
    let mut result: String = String::from("");
    // Initialize our available_chars variable, which will contain either letters, number and/or symbols
    let mut available_chars: String = String::from("");
    // Append letters to available_chars
    available_chars.push_str(&get_letters());
    // Append symbols to available_chars if use_special_chars is true
    if use_special_chars {
        available_chars.push_str(&get_symbols());
    }
    // Append numbers to available_chars
    available_chars.push_str(&get_number());
    // Get final result
    for _i in 0..length {
        // Generate a random number between 0 and length of available_chars (~26+10+10)
        let rand_int: usize = rng.gen_range(0..available_chars.len());
        // Get character from available_chars string at rand_int index
        let mut character: String = available_chars.chars().nth(rand_int).unwrap().to_string();
        let zero_or_one: usize = rng.gen_range(0..2);
        if zero_or_one == 1 {
            character = character.to_uppercase();
        }
        // Append character to result string
        result.push_str(&character);
    }
    return result;
}

fn generate_secret_ascii(length: i32) -> String {
    let mut rng: ThreadRng = rand::thread_rng();
    // Initialize our result variable, which this function will return
    let mut result: String = String::from("");
    let combined_iter = ascii::get_lowercase_letter()
        .chain(ascii::get_uppercase_letter())
        .chain(ascii::get_number())
        .chain(ascii::get_special_character());
    let combined_vector: Vec<i32> = combined_iter.collect();
    // Get final result
    for _i in 0..length {
        let rand_index = rng.gen_range(0..combined_vector.len());
        result.push_str(&(combined_vector[rand_index] as u8 as char).to_string());
    }
    return result;
}

fn main() {
    // Generate secret
    let mut start = Instant::now();
    let secret: String = generate_secret(10, true);
    let mut duration = start.elapsed();
    // Print result to stdout
    println!("Secret: {secret} // Execution time: {:?}", duration);
    // Using ASCII
    start = Instant::now();
    let ascii_secret: String = generate_secret_ascii(10);
    duration = start.elapsed();
    println!(
        "Secret (ascii): {ascii_secret} // Execution time: {:?}",
        duration
    )
}
