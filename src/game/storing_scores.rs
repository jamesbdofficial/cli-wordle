// Function to store the user's scores to a text file.

use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;

pub fn save_score(attempts: u8) {
    let dt = Utc::now();
    let mut data: String = "".to_string();
    data.push_str(&attempts.to_string());
    data.push_str(", ");
    data.push_str(&dt.to_string());

    let mut scores_file = OpenOptions::new()
        .append(true)
        .read(true)
        .open("src/text_files/scores_list.txt")
        .expect("File isn't opening");
    write!(&mut scores_file, "{} \n", data); // writes attempt number and date time to file.
}
