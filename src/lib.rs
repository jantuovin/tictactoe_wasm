use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mark {
    Empty = 0,
    Nought = 1,
    Cross = 2,
}

#[wasm_bindgen]
pub struct TicTacToe {
    board_size: u8,
    marks_needed_for_win: u8,
    mark_in_turn: Mark,
    cells: Vec<Mark>,
}

// public
#[wasm_bindgen]
impl TicTacToe {
    pub fn new(board_size: u8, marks_needed_for_win: u8, mark_in_turn: Mark) -> TicTacToe {
        let cells = (0..board_size * board_size).map(|_i| Mark::Empty).collect();
        TicTacToe {
            board_size,
            marks_needed_for_win,
            cells,
            mark_in_turn,
        }
    }

    pub fn get_board(&self) -> String {
        self.to_string()
    }

    pub fn whos_turn(&self) -> Mark {
        self.mark_in_turn
    }

    pub fn get_turn_mark(&self) -> char {
        TicTacToe::get_mark_symbol(self.mark_in_turn)
    }

    pub fn make_move(&mut self, row: u8, col: u8, mark: Mark) {
        let idx = self.get_cell_index(row, col);
        if self.cells[idx] == Mark::Empty {
            self.cells[idx] = mark;
            self.mark_in_turn = match mark {
                Mark::Nought => Mark::Cross,
                _ => Mark::Nought,
            }
        }
    }

    pub fn get_winner(&self) -> char {
        TicTacToe::get_mark_symbol(self.get_winner_mark())
    }
}

// private
impl TicTacToe {
    fn get_mark_symbol(mark: Mark) -> char {
        match mark {
            Mark::Nought => '0',
            Mark::Cross => 'X',
            _ => ' ',
        }
    }

    fn get_cell_index(&self, row: u8, col: u8) -> usize {
        (self.board_size * row + col) as usize
    }

    fn get_winner_mark(&self) -> Mark {
        // check rows
        for row in 0..self.board_size {
            let winning_mark = self.search_cell_row_for_winner(row, 0, 0, 1);
            if winning_mark != Mark::Empty {
                return winning_mark;
            }
        }

        //check cols
        for col in 0..self.board_size {
            let winning_mark = self.search_cell_row_for_winner(0, col, 1, 0);
            if winning_mark != Mark::Empty {
                return winning_mark;
            }
        }

        // check diagonals
        for row in 0..self.board_size {
            for col in 0..self.board_size {
                let mut winning_mark = self.search_cell_row_for_winner(row, col, 1, 1);
                if winning_mark != Mark::Empty {
                    return winning_mark;
                }

                winning_mark = self.search_cell_row_for_winner(row, col, 1, -1);
                if winning_mark != Mark::Empty {
                    return winning_mark;
                }
            }
        }

        Mark::Empty
    }

    fn search_cell_row_for_winner(
        &self,
        row: u8,
        col: u8,
        row_delta_factor: i8,
        col_delta_factor: i8,
    ) -> Mark {
        let mut count = 0;
        let mut prev_mark = Mark::Empty;

        let mut x = row as i8;
        let mut y = col as i8;
        loop {
            if !((0 <= x && x < self.board_size as i8) && (0 <= y && y < self.board_size as i8)) {
                break;
            }

            let idx = self.get_cell_index(x as u8, y as u8);
            if self.cells.len() - 1 < idx as usize {
                break;
            }

            if prev_mark == self.cells[idx] && self.cells[idx] != Mark::Empty {
                count = count + 1;
            } else {
                count = 1;
                prev_mark = self.cells[idx];
            }

            if count == self.marks_needed_for_win {
                return self.cells[idx];
            }

            x = x + (1 * row_delta_factor);
            y = y + (1 * col_delta_factor);
        }

        Mark::Empty
    }
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.board_size as usize) {
            for &cell in line {
                write!(f, "{}", TicTacToe::get_mark_symbol(cell))?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
