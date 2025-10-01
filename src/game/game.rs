// Main game file.

use super::compare_words;
use super::get_input;
use super::pick_target_word;
use super::print_result;
use super::storing_scores;
use super::validate_word;
use super::word_list_to_vector;

pub fn game() {
    // Initialising the vectors for later use.
    let mut target_word_vector: Vec<char> = vec![];
    let mut guess_result_vector: Vec<u8> = vec![];
    let mut guess_word_vector: Vec<char> = vec![];

    // Conditions for game over.
    let mut solved: bool = false;
    let mut attempts: u8 = 0;

    // Set up of word list and picking word from that, then converting the target word to a vector (for letter comparison).
    let word_list_vector: Vec<String> = word_list_to_vector::text_to_vec();
    let target_word = pick_target_word::pick_word(word_list_vector.clone());
    target_word_vector = target_word.chars().collect();

    // Greeting.
    println!();
    println!("Welcome to CLI Wordle! Guess the word!");
    println!();

    // User still has more attempts remaining and has not guessed the word correctly.
    while attempts < 6 && !solved {
        let mut guess_word = get_input::get_input();
        guess_word = guess_word.to_lowercase();
        guess_word_vector = guess_word.chars().collect();
        let word_length = guess_word.len();
        let word_in_list = validate_word::word_in_word_list(word_list_vector.clone(), guess_word);
        println!();

        // Validation for guess (user cannot guess a word that isn't in the list.)
        if word_length == 5 && word_in_list {
            guess_result_vector =
                compare_words::compare(guess_word_vector.clone(), target_word_vector.clone());
            print_result::result(guess_word_vector.clone(), guess_result_vector);
            if guess_word_vector == target_word_vector {
                solved = true;
            }
            attempts += 1;
            println!(
                "Attempts: {}. You have {} attempts remaining",
                attempts,
                6 - attempts
            );
            println!();
        } else if word_length != 5 {
            println!("The word must be 5 letters in length.");
        } else {
            println!("This is not a word we know! Please try again");
            println!();
        }
    }

    if solved == true {
        println!("Well done! You solved this in {} attempts!", attempts);
        storing_scores::save_score(attempts);
    } else if attempts >= 6 {
        println!(
            "Unlucky! The word was {}. Better luck next time!",
            target_word
        );
    } else {
        println!("This game is broken. Please contact devs with information about how this happened and they will fix it ASAP.");
    }
    println!();
}
