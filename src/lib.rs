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
            let player_icon = if self.player_one_turn { "X" } else { "O" };

            if let Ok(int_input) = input.parse::<usize>() {
                println!("intput = {}", int_input);

                let row = (int_input - 1) / self.board.len();
                let col = (int_input - 1) % self.board.len();

                println!("row = {} col = {}", row, col);
                self.set_position(row, col, player_icon);
            } else {
                println!("Invalid input: \"{}\" is not a number between 0-9", input)
            }

            // match input {
            //     "1" => self.set_position(0, 0, player_icon),
            //     "2" => self.set_position(0, 1, player_icon),
            //     "3" => self.set_position(0, 2, player_icon),
            //     "4" => self.set_position(1, 0, player_icon),
            //     "5" => self.set_position(1, 1, player_icon),
            //     "6" => self.set_position(1, 2, player_icon),
            //     "7" => self.set_position(2, 0, player_icon),
            //     "8" => self.set_position(2, 1, player_icon),
            //     "9" => self.set_position(2, 2, player_icon),
            //     _ => println!("Invalid input"),
            // }
        }
    }

    pub fn set_position(&mut self, row: usize, col: usize, icon: &str) {
        self.board[row][col] = icon.to_string();
        self.player_one_turn = !self.player_one_turn;
    }
}

/*
input = 4
row = 3 / 3 = 1
col = 3 % 3 = 0

input = 2
row = 2 % 3 = 2
col = 2 / 3 = 0

input = 3
row = 2 / 3 = 0
col = 2 % 3 = 2
ans = row = 0, col = 2

*/
