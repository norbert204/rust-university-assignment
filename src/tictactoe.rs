pub type Grid = [BoardGridCell; 9];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoardGridCell {
    None,
    Circle,
    Cross,
}

impl BoardGridCell {
    pub fn to_board_string(&self, tile: i32) -> String {
        match self {
            BoardGridCell::None => tile.to_string(),
            BoardGridCell::Cross => "X".to_owned(),
            BoardGridCell::Circle => "O".to_owned(),
        }
    }
}

impl ToString for BoardGridCell {
    fn to_string(&self) -> String {
        match self {
            BoardGridCell::None => " ".to_owned(), 
            BoardGridCell::Cross => "X".to_owned(),
            BoardGridCell::Circle => "O".to_owned(),
        }
    }
}

pub fn check_winner(cells: &Grid) -> Option<BoardGridCell> {
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

    if cells.iter().all(|x| *x != BoardGridCell::None) {
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
        let result = check_series(&cells, start, offset, 3);

        if let Some(x) = result {
            return Some(x);
        }
    }

    None
}
