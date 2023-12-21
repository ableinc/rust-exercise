use rand::{Rng, rngs::ThreadRng};

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
        let character: String = available_chars.chars().nth(rand_int).unwrap().to_string();
        // Append character to result string
        result.push_str(&character);
    }
    return result;
}

fn main() {
    // Generate secret
    let secret: String = generate_secret(10, true);
    // Print result to stdout
    println!("Secret: {secret}");
}
