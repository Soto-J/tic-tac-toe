use std::io;

use tic_tac_toe::Game;

fn main() {
    let mut game = Game::new(3, 3);

    let mut winning_player = String::new();

    while winning_player.len() == 0 {
        game.draw();
        print!("Choose a position: ");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        println!("Input: {}", user_input);

        game.position(Some(user_input.trim()));
    }
}
