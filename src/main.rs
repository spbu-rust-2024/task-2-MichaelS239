use std::cmp::min;
use std::io;

fn find_max_odd_palindrome(string: &String) -> String {
    let bytes = string.as_bytes();
    let mut max_palindrome = &string[0..0];
    for index in 0..string.len() {
        let mut palindrome = &string[index..index + 1];
        for offset in 1..min(index + 1, string.len() - index) {
            if bytes[index - offset] != bytes[index + offset] {
                palindrome = &string[index - offset + 1..index + offset];
                break;
            }
            if index - offset == 0 || index + offset == string.len() - 1 {
                palindrome = &string[index - offset..index + offset + 1];
                break;
            }
        }
        if palindrome.len() > max_palindrome.len() {
            max_palindrome = palindrome;
        }
    }
    max_palindrome.to_string()
}

fn find_max_even_palindrome(string: &String) -> String {
    let bytes = string.as_bytes();
    let mut max_palindrome = &string[0..0];
    if string.len() == 0 {
        return max_palindrome.to_string();
    }
    for index in 0..string.len() - 1 {
        let mut palindrome = &string[index..index];
        for offset in 1..min(index + 2, string.len() - index) {
            if bytes[index + 1 - offset] != bytes[index + offset] {
                palindrome = &string[index + 2 - offset..index + offset];
                break;
            }
            if index + 1 - offset == 0 || index + offset == string.len() - 1 {
                palindrome = &string[index + 1 - offset..index + offset + 1];
                break;
            }
        }
        if palindrome.len() > max_palindrome.len() {
            max_palindrome = palindrome;
        }
    }
    max_palindrome.to_string()
}

fn find_max_palindrome(string: &String) -> String {
    let max_odd_palindrome = find_max_odd_palindrome(string);
    let max_even_palindrome = find_max_even_palindrome(string);
    if max_odd_palindrome.len() > max_even_palindrome.len() {
        max_odd_palindrome
    } else {
        max_even_palindrome
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("{}", find_max_palindrome(&input.trim().to_string()));
}
