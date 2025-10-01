// Function for getting the user input and returning it to main.

pub fn get_input() -> String{
    use std::io::{stdin,stdout,Write};
    let mut user_word=String::new();
    println!("Input: ");
    let _=stdout().flush();
    stdin().read_line(&mut user_word).expect("You did not enter a valid word");
    if let Some('\n')=user_word.chars().next_back() {
        user_word.pop();
    }
    if let Some('\r')=user_word.chars().next_back() {
        user_word.pop();
    }
    return user_word;
}