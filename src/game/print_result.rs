// Printing results with colour depending on if the letter is in the correct place and how many times the letter is in the word .

pub fn result(guess_word_vector: Vec<char>, guess_result_vector: Vec<u8>) {
    let mut i = 0;
    while i < 5 {
        if guess_result_vector[i] == 0 {
            colour::red!("{}", guess_word_vector[i]);
        } else if guess_result_vector[i] == 1 {
            colour::yellow!("{}", guess_word_vector[i]);
        } else if guess_result_vector[i] == 2 {
            colour::cyan!("{}", guess_word_vector[i]);
        } else if guess_result_vector[i] == 3 {
            colour::green!("{}", guess_word_vector[i]);
        } else if guess_result_vector[i] == 4 {
            colour::magenta!("{}", guess_word_vector[i]);
        } else {
            println!("This game is broken. Please contact devs with information about how this happened and they will fix it ASAP.");
        }
        print!(" ");
        i += 1;
    }
    println!("");
}
