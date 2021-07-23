/*
    password.rs - Functions to create the password.

    (C) 2020 Tim Gravert <crazymind102@googlemail.com>

    License: MIT OR Apache-2.0
*/


use heck::TitleCase;
use rand::Rng;
use rand::rngs::OsRng;
use std::mem::swap;



/// Function to create a easy to remember password.
/// It takes the wordlist, two seperators and the number which is the number of words in the
/// password.
pub fn create_password(wordlist: Vec<String>, s1: String, s2: String, n: usize) -> String {
    let chosen = choose_words(wordlist, n);
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
        swap(&mut s1, &mut s2);
    }
    password
}


/// Random word collector.
/// Creates a list of random words with the help of a list of random numbers which are cryptograhically secure.
/// The first letter of each word gets capitalized to add to the security of the password.
pub fn choose_words(words: Vec<String>, n: usize) -> Vec<String> {
    let mut rng = OsRng::new().expect("error creating OsRng");
    let mut numbers: Vec<usize> = Vec::new();
    numbers.resize_with(n, || rng.gen_range(0,words.len()));
    let mut chosen: Vec<String> = Vec::new();
    for i in numbers {
        let temp = &words[i];
        let temp = temp.to_title_case();
        chosen.push(temp);
    }
    chosen
}
