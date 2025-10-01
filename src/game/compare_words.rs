// Function for comparing guess word to target word.

pub fn compare(guess_word_vector: Vec<char>, target_word_vector: Vec<char>) -> Vec<u8> {
    let mut guess_result_vec: Vec<u8> = vec![0, 0, 0, 0, 0];

    let mut i1 = 0;
    let mut i2 = 0;
    let mut number_of_times_in_word = 0;

    while i1 < 5 {
        while i2 < 5 {
            if guess_word_vector[i1] == target_word_vector[i2] {
                number_of_times_in_word += 1;
            }
            i2 += 1;
        }

        if guess_word_vector[i1] == target_word_vector[i1] && number_of_times_in_word > 1 {
            guess_result_vec[i1] = 4;
        } else if guess_word_vector[i1] == target_word_vector[i1] && number_of_times_in_word == 1 {
            guess_result_vec[i1] = 3;
        } else if target_word_vector.contains(&guess_word_vector[i1]) && number_of_times_in_word > 1
        {
            guess_result_vec[i1] = 2;
        } else if target_word_vector.contains(&guess_word_vector[i1])
            && number_of_times_in_word == 1
        {
            guess_result_vec[i1] = 1;
        } else {
            guess_result_vec[i1] = 0;
        }

        number_of_times_in_word = 0;
        i2 = 0;
        i1 += 1;
    }

    return guess_result_vec;

    //value of guess_result_vec can be 0, 1, 2, 3 or 4
    // 0 = not in word
    // 1 = in word   once   and   not   in right place
    // 2 = in word > once   and   not   in right place
    // 3 = in word   once   and         in right place
    // 4 = in word > once   and         in right place
}
