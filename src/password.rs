/*
    password.rs - Functions to create the password.

    (C) 2020 Tim Gravert <crazymind102@googlemail.com>

    License: MIT OR Apache-2.0
*/


use heck::TitleCase;
use rand::RngCore;
use rand::rngs::OsRng;



/// Function to create a easy to remember password.
/// It takes the wordlist, two seperators and the number which is the number of words in the
/// password.
pub fn create_password(wordlist: Vec<String>, s1: String, s2: String, n: usize) -> String {
    let numbers = random_numbers(n);
    let chosen = choose_words(wordlist, numbers);
    let password = password(chosen, s1, s2);
    password
}


/// Password creation function.
/// Two Strings get interlaced between a list of Strings to form a secure easy to remember
/// password.
pub fn password(words: Vec<String>, mut s1: String, mut s2: String) -> String {
    let mut password = String::new();
    for mut word in words {
        word.push_str(&s1);
        password = password + &word;
        let temp = s1;
        s1 = s2;
        s2 = temp;
    }
    password
}


/// Auxiliary function for random numbers.
/// Creates a list of random numbers. The size of the list is the input.
/// The numbers are created by the csprng of the operating system.
/// The variable n is the number of numbers we want from the function.
pub fn random_numbers(mut n: usize) -> Vec<usize> {
    let mut numbers: Vec<usize> = Vec::new();
    match OsRng::new() {
        Err(err) => panic!("error: {:?}", err),
        Ok(mut f) =>
            while n > 0 {
                numbers.push(OsRng::next_u64(&mut f) as usize);
                n -= 1;
            },
    };
    numbers
}


/// Random word collector.
/// Creates a list of random words with the help of a list of random numbers which should be
/// cryptograhically secure.
/// The first letter of each word gets capitalized to add to the security of the password.
pub fn choose_words(words: Vec<String>, numbers: Vec<usize>) -> Vec<String> {
    let mut chosen: Vec<String> = Vec::new();
    for i in numbers {
        let temp = &words[i%words.len()];
        let temp = temp.to_title_case();
        let temp = String::from(temp);
        chosen.push(temp);
    }
    chosen
}
