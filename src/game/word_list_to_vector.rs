// Function for converting the list of words to a vector, for use when validating the word.

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String>{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

pub fn text_to_vec() -> Vec<String>{
    let lines = lines_from_file("src/text_files/word_list.txt");
    return lines;
}