pub const COLUMNS: usize = 4;
pub const ROWS: usize = 5;

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Marked(u8),
    UnMarked(u8),
}

impl Cell {
    pub fn is_marked(&self) -> bool {
        match self {
            Cell::Marked(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    cells: Vec<Cell>,
}

impl Board {
    pub fn mark(&self, num: u8) -> Board {
        let cells = self
            .cells
            .iter()
            .copied()
            .map(|cell| match cell {
                Cell::UnMarked(n) if n == num => Cell::Marked(num),
                _ => cell,
            })
            .collect();
        Board { cells }
    }

    pub fn has_won(&self) -> bool {
        (0..COLUMNS).any(|col| self.is_complete_column(col))
            || (0..ROWS).any(|row| self.is_complete_row(row))
    }

    /// Get an immutable reference of the value on matrix[(i, j)]
    /// **Return**: An Option that returns None if the index is out of range
    ///             or the reference within a [`Some`] variant.
    pub fn get(&self, i: usize, j: usize) -> Option<&Cell> {
        self.cells.get(i * COLUMNS + j)
    }

    fn is_complete_column(&self, column: usize) -> bool {
        (0..ROWS)
            .map(|row| self.get(row, column))
            .all(|cell| cell.map_or(false, |cell| cell.is_marked()))
    }

    fn is_complete_row(&self, row: usize) -> bool {
        (0..COLUMNS)
            .map(|col| self.get(row, col))
            .all(|cell| cell.map_or(false, |cell| cell.is_marked()))
    }
}

impl From<&str> for Board {
    fn from(str_board: &str) -> Self {
        let cells: Vec<Cell> = str_board
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .map(Cell::UnMarked)
            .collect();
        Board { cells }
    }
}
