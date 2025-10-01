// Function for checking that the word is a valid word (must be contained within the list of words).

pub fn word_in_word_list(word_list: Vec<String>, user_word: String) -> bool {
    let mut word_in_list = false;
    
    for line in word_list{
        if user_word == line {
            word_in_list = true;
            break;
        }
    }    
    return word_in_list;
}