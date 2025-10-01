// Function for Displaying all previous scores.
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

pub fn show_scores() {
    let lines = lines_from_file("src/text_files/scores_list.txt");
    for line in lines {
        println!("{}", line)
    }
}