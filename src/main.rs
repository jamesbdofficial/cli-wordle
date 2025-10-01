// Command Line Wordle Game - First Rust Project.
// Author - James Brozicevic - Doran.

pub mod game;
pub mod scores {
    pub mod show_scores;
}
use crate::scores::show_scores;
mod command_print;

fn main() {
    let mut quit = false;
    let mut command: String = "".to_string();

    println!("-------------------------------------------------------");
    println!("Welcome to CLI Wordle!");
    println!("To see a list of possible commands, type 'help'.");
    command_print::print_commands();
    println!("I hope you enjoy!");
    println!("-------------------------------------------------------");

    while quit == false {
        command = game::get_input::get_input().to_lowercase();

        if command == "play" {
            game::game::game();
        } else if command == "scores" {
            show_scores::show_scores();
        } else if command == "help" {
            command_print::print_commands();
        } else if command == "quit" {
            println!("Thank you for playing! Have a good day!");
            quit = true;
        } else {
            println!(
                "This command is not recognised, type 'help' to see a list of possible commands."
            )
        }
    }
}
