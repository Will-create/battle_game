use std::io::{ self, Write};
use rand;


const BOARD_SIZE: usize = 10;

struct Board {
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<(usize, usize)>,

}

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    Ship,
    Hit,
    Miss
}

impl Board {
    fn new() -> Self {
        Board {
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            ships: Vec::new()
        }
    }


    // Function to randomly place a ship on the board
    fn place_ship (&mut self, size: usize) {
        let mut range = rand::thread_rng();
        loop {
            let row = range.gen_range(0..BOARD_SIZE);
            let col = range.gen_range(0..BOARD_SIZE);
            let direction = range.gen::<bool>();
            if self.can_place_ship(row, col, size, direction) {
                for i in 0..size {
                    let (r, c) = if direction {
                        (row, col + i)
                    } else {
                        (row + i, col)
                    };

                    self.grid[r][c] = CellState::Ship;
                    self.ships.push((r, c));
                }
            }
        }
    }

    fn can_place_ship(&self, row: usize, col: usize, size: usize, direction: bool) -> bool {
    // check if the ship can be placed on the board
        if direction {
            if col + size > BOARD_SIZE { return false;}

            for i in 0..size {
                if self.grid[row][col + i] != CellState::Empty { return false;}
            }
        } else {
            if row + size > BOARD_SIZE  { return false;}
            for i in 0..size {
                if self.grid[row + i][col] != CellState::Empty { return false;}
            }
        }  
        true
    }

    // Method to fire
    fn fire(&mut self, row: usize, col: usize) -> CellState {
        match self.grid[row][col] {
            CellState::Empty => {
                self.grid[row][col] = CellState::Miss
                CellState::Miss
            },
            CellState::Ship => {
                self.grid[row][col] = CellState::Hit;
                CellState::Hit
            },
            _ => CellState::Miss
        }
    }

    // Method to display the game board

    fn display (&self, hide_ships: bool) {
        println!("  ");
        for i in 0..BOARD_SIZE { println!(" {} ", i)}
        println!();
        for (i, _row ) in self.grid.iter().enumerate() {
            print!("{:2} ", i);  

            for cell in _row {
                match cell {
                    CellState::Empty => print!(" \u{25A1} "),
                    CellState::Ship => {
                        if hide_ships { print!("    "); } else { print!(" \u{25A0} ");}
                    },
                    CellState::Hit => print!("\x1b[31m \u{25CF} \x1b[0m"),
                    CellState::Miss => print!("\x1b[36m \u{00B7} \x1b[0m")

                }
                println!()
            }
        }
    }

    // Method for game over
    fn is_game_over(&self) -> bool {
        self.ships.iter().all(|&(r, c)| self.grid[r][c] == CellState::Hit)
    }

}





fn main() {
    loop {

    }
}


fn get_player_input() {

}

fn generate_oponent_move() {

}