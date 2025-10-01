// Function for picking a random word from the list of 5,700 5 letter words.

use rand::prelude::*;

pub fn pick_word(words: Vec<String>) -> String {
    let mut rng = rand::rng();
    let random_number: u32 = rng.random_range(0..5757);
    let target_word: String = words[random_number as usize].clone();
    return target_word;
}
