/*
 * password.rs - Functions to create the password.
 *
 * (C) 2020 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

use rand::Rng;
use std::mem::swap;

/* 
 * Function to create a easy to remember password.
 * It takes the wordlist, two seperators and the number which is the number of words in the
 * password.
 *
 */
pub fn create_password(wordlist: &mut Vec<String>, s1: String, s2: String, n: usize) -> String {
    let chosen = choose_words(wordlist, n);
    password(chosen, s1, s2)
}

/*
 * Password creation function.
 * Two Strings get interlaced between a list of Strings to form a secure easy to remember
 * password.
 *
 */
pub fn password(words: Vec<String>, mut s1: String, mut s2: String) -> String {
    let mut password = String::new();
    for mut word in words {
        word.push_str(&s1);
        password = password + &word;
        swap(&mut s1, &mut s2);
    }
    password
}

/*
 * Random word collector.
 * Creates a vector of Strings which are randomly chosen by the cryptographical secure random number
 * generator of the OS.
 * The Strings are chosen from a vector of Strings.
 * The first letter of each String gets capitalized to add to the security of the password.
 *
 */
pub fn choose_words(words: &mut Vec<String>, n: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut chosen: Vec<String> = Vec::with_capacity(n);
    //words = words.into_iter().map(|x| x.trim().to_string()).collect::<Vec<String>>();
    chosen.resize_with(n, || {
        words.remove(rng.gen_range(0..words.len()))
    });
    chosen
}
