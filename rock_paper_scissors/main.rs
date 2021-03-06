extern crate rand;
use std::io;
use rand::prelude::*;


fn main() {
    let mut ai_wins = 0;
    let mut wins = 0;
    loop {
        println!("\nWins: {} Losses: {}\nChoose your move!\nRock, Paper or Scissor\r\n", wins, ai_wins);
        let mut user_move = String::new();
        io::stdin()
            .read_line(&mut user_move)
            .expect("Failed to read input");

        let mut rng = thread_rng();
        let ai_choice: u8 = rng.gen_range(0, 3);
        let mut ai_move = String::from("Null");
        if ai_choice == 0 {
            ai_move = "Rock".to_string();
        } else if ai_choice == 1 {
            ai_move = "Paper".to_string();
        } else if ai_choice == 2 {
            ai_move = "Scissor".to_string();
        }

        let len = user_move.len();
        let user_move_without = user_move.trim_right().len();
        user_move.truncate(user_move_without);

        if user_move.to_lowercase() == ai_move.to_lowercase() {
            println!("Tie!")
        } else if user_move.to_lowercase() == "rock" && ai_move.to_lowercase() == "scissor" {
            println!("You won! Opponent chose {}", ai_move);
            wins += 1;
        } else if user_move.to_lowercase() == "paper" && ai_move.to_lowercase() == "rock" {
            println!("You won! Opponent chose {}", ai_move);
            wins += 1;
        } else if user_move.to_lowercase() == "scissor" && ai_move.to_lowercase() == "paper" {
            println!("You won! Opponent chose {}", ai_move);
            wins += 1;
        } else if user_move.to_lowercase() == "rock" && ai_move.to_lowercase() == "paper" {
            println!("You lost! Opponent chose {}", ai_move);
            ai_wins += 1;
        } else if user_move.to_lowercase() == "paper" && ai_move.to_lowercase() == "scissor" {
            println!("You lost! Opponent chose {}", ai_move);
            ai_wins += 1;
        } else if user_move.to_lowercase() == "scissor" && ai_move.to_lowercase() == "rock" {
            println!("You lost! Opponent chose {}", ai_move);
            ai_wins += 1;
        } else {
            println!("You did not enter a correct choice!");
        }

        if wins == 4 {
            println!("\nYou won the game! You: 4 Opponent: {}\n", ai_wins);
            wins = 0;
            ai_wins = 0;
        } else if ai_wins == 4 {
            println!("\nYou lost the game! You: {} Opponent: 4\n", wins);
            wins = 0;
            ai_wins = 0;
        }
    }

}
