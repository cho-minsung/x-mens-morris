#[derive(Clone)]
pub struct Move {
    // Move is a human-readible symantic move record
    pub col: char,
    pub row: u8,
    pub new_col: Option<char>,
    pub new_row: Option<u8>,
}

impl Move {
    pub fn new() -> Move {
        Move {
            col: 'z',
            row: 0,
            new_col: None,
            new_row: None
        }
    }
    pub fn place_new_piece(row: &usize, col: &usize) -> Self {
        let row = *row as u8 + 1;
        let col = match col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            _ => ' ',
        };

        Self {
            col: col,
            row: row,
            new_col: None,
            new_row: None,
        }
    }

    pub fn move_piece(row: &usize, col: &usize, new_row: &usize, new_col: &usize) -> Self {
        let col = match col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            _ => ' ',
        };
        let row = *row as u8 + 1;

        let new_col = match new_col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            _ => ' ',
        };
        let new_row = *new_row as u8 + 1;

        Self {
            col: col,
            row: row,
            new_col: Some(new_col),
            new_row: Some(new_row),
        }
    }

    pub fn is_new_move(&self) -> bool {
        return self.new_row.is_none();
    }

    pub fn new_as_coord(&self) -> (usize, usize) {
        match self.col {
            'A' | 'a' => return (self.row as usize - 1, 0),
            'B' | 'b' => return (self.row as usize - 1, 1),
            'C' | 'c' => return (self.row as usize - 1, 2),
            _ => return (3, 3),
        }
    }

    pub fn move_as_coord(&self) -> (usize, usize) {
        if self.is_new_move() {
            return (3, 3);
        };
        match self.new_col.unwrap() {
            'A' | 'a' => return (self.new_row.unwrap() as usize - 1, 0),
            'B' | 'b' => return (self.new_row.unwrap() as usize - 1, 1),
            'C' | 'c' => return (self.new_row.unwrap() as usize - 1, 2),
            _ => return (3, 3),
        }
    }
}