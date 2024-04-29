const TOTAL_ROWS: usize = 3;

pub struct Game {
    pub board: Vec<Vec<String>>,
    pub player_one_turn: bool,
}
impl Game {
    pub fn new(rows: usize, cols: usize) -> Game {
        println!("Welcome to tic-tac-toe");

        let mut board = Vec::with_capacity(rows);
        let mut counter = 1;

        for i in 1..=rows {
            let mut row = Vec::with_capacity(cols);

            for j in 1..=cols {
                row.push(format!("{}", counter));
                counter += 1;
            }

            board.push(row);
        }

        Game {
            board,
            player_one_turn: true,
        }
    }

    pub fn draw(&self) {
        let mut display = String::new();

        for (i, row) in self.board.iter().enumerate() {
            display.push_str(&row.join(" | "));
            display.push('\n');

            if i != self.board.len() - 1 {
                display.push_str("  -   -\n");
            }
        }

        println!("Game:\n\n{}", display)
    }

    pub fn position(&mut self, user_input: Option<&str>) {
        if let Some(input) = user_input {
            match input.parse::<usize>() {
                Ok(int_input)
                    if int_input >= 1 && int_input <= self.board.len() * self.board.len() =>
                {
                    let player_icon = if self.player_one_turn { "X" } else { "O" };

                    let row = (int_input - 1) / self.board.len();
                    let col = (int_input - 1) % self.board.len();

                    self.set_position(row, col, player_icon);
                }

                _ => println!("Invalid input: \"{}\" is not a number between 0-9", input),
            };
        }
    }

    pub fn set_position(&mut self, row: usize, col: usize, icon: &str) {
        self.board[row][col] = icon.to_string();
        self.player_one_turn = !self.player_one_turn;
    }
}

/*
input = 4
row = (input - 1) / 3 = 1
col = (input - 1) % 3 = 0
*/
