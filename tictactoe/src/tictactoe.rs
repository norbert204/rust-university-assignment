use std::{error::Error, fmt::Display};

use rand::Rng;

use crate::grid::{BoardGridCell, Grid};

pub struct TicTacToe {
    cells: Grid,
}

impl Default for TicTacToe {
    fn default() -> Self {
        Self {
            cells: [BoardGridCell::None; 9],
        }
    }
}

impl TicTacToe {
    pub fn player_step(&mut self, cell: i32) -> Result<(), TicTacToeErrors> {
        if cell < 0 || cell >= self.cells.len() as i32
        {
            return Err(TicTacToeErrors::StepOutOfBounds);
        }

        let cell = cell as usize;

        match self.cells[cell] {
            BoardGridCell::None => {
                self.cells[cell] = BoardGridCell::Circle;

                Ok(())
            },
            _ => Err(TicTacToeErrors::CellAlreadyOccupied),
        }
    }

    pub fn enemy_step(&mut self) {
        let mut rng = rand::thread_rng();

        loop {
            let index = rng.gen_range(0..self.cells.len());

            if self.cells[index] == BoardGridCell::None {
                self.cells[index] = BoardGridCell::Cross;
                break;
            }
        }
    }

    pub fn check_winner(&self) -> Option<BoardGridCell> {
        fn check_series(cells: &Grid, starting_pos: usize, offset: usize, needed_matches: i32) -> Option<BoardGridCell> {
            let value = cells[starting_pos];

            if value == BoardGridCell::None {
                return None;
            }

            for i in 0..needed_matches {
                if cells[starting_pos + i as usize * offset] != value {
                    return None;
                }
            }

            return Some(value);
        }

        if self.cells.iter().all(|x| *x != BoardGridCell::None) {
            return Some(BoardGridCell::None);
        }

        let starts_and_offsets = [
            ( 0, 1 ),
            ( 3, 1 ),
            ( 6, 1 ),
            ( 0, 3 ),
            ( 1, 3 ),
            ( 2, 3 ),
            ( 0, 4 ),
            ( 2, 2 ),
        ];

        for (start, offset) in starts_and_offsets {
            let result = check_series(&self.cells, start, offset, 3);

            if let Some(x) = result {
                return Some(x);
            }
        }

        None
    }

    pub fn cells(&self) -> Grid {
        self.cells
    }
}

#[derive(Debug)]
pub enum TicTacToeErrors {
    StepOutOfBounds,
    CellAlreadyOccupied,
}

impl Error for TicTacToeErrors {}

impl Display for TicTacToeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TicTacToeErrors::StepOutOfBounds => write!(f, "Given cell index is out of bounds!"),
            TicTacToeErrors::CellAlreadyOccupied => write!(f, "Cell at given index is already occupied!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_winner_circle_row() {
        for i in (0..9).step_by(3) {
            // Arrange
            let mut game = TicTacToe::default();

            game.cells[i + 0] = BoardGridCell::Circle;
            game.cells[i + 1] = BoardGridCell::Circle;
            game.cells[i + 2] = BoardGridCell::Circle;

            // Act
            let result = game.check_winner();

            // Assert
            assert_eq!(result, Some(BoardGridCell::Circle));
        }
    }

    #[test]
    fn check_winner_circle_column() {
        for i in 0..3 {
            // Arrange
            let mut game = TicTacToe::default();

            game.cells[i + 0] = BoardGridCell::Circle;
            game.cells[i + 3] = BoardGridCell::Circle;
            game.cells[i + 6] = BoardGridCell::Circle;

            // Act
            let result = game.check_winner();

            // Assert
            assert_eq!(result, Some(BoardGridCell::Circle));
        }
    }

    #[test]
    fn check_winner_circle_diagonal() {
        // Arrange
        let mut game = TicTacToe::default();

        for i in (0..9).step_by(4) {
            game.cells[i] = BoardGridCell::Circle;
        }

        // Act
        let result = game.check_winner();

        // Assert
        assert_eq!(result, Some(BoardGridCell::Circle));
    }

    #[test]
    fn check_winner_circle_backward_diagonal() {
        // Arrange
        let mut game = TicTacToe::default();

        for i in (2..9).step_by(2) {
            game.cells[i] = BoardGridCell::Circle;
        }

        // Act
        let result = game.check_winner();

        // Assert
        assert_eq!(result, Some(BoardGridCell::Circle));
    }
}
