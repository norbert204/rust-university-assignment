pub type Grid = [BoardGridCell; 9];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoardGridCell {
    None,
    Circle,
    Cross,
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
