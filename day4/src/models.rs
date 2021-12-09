pub const COLUMNS: usize = 5;
pub const ROWS: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Marked(u16),
    UnMarked(u16),
}

impl Cell {
    pub fn is_marked(&self) -> bool {
        match self {
            Cell::Marked(_) => true,
            _ => false,
        }
    }

    pub fn into_inner(&self) -> u16 {
        match self {
            Cell::Marked(num) => *num,
            Cell::UnMarked(num) => *num,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    cells: Vec<Cell>,
}

impl Board {
    pub fn mark(&self, num: u16) -> Board {
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

    pub fn get_score(&self) -> u16 {
        self.cells
            .iter()
            .filter(|cell| !cell.is_marked())
            .map(|cell| cell.into_inner())
            .sum::<u16>()
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

#[cfg(test)]
mod tests {
    use super::{Board, Cell};

    const BOARD_STR: &'static str = r#"
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7"#;

    #[test]
    fn creates_board_from_str() {
        let nums: [u16; 25] = [
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ];
        let expected_cells: Vec<Cell> = nums.iter().copied().map(Cell::UnMarked).collect();
        let board = Board::from(BOARD_STR);
        assert_eq!(expected_cells, board.cells);
    }

    #[test]
    fn marks_cell() {
        let num_to_mark = 14;
        let nums: [u16; 25] = [
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ];
        let board = Board::from(BOARD_STR).mark(num_to_mark);
        let marked_cells: Vec<Cell> = board
            .cells
            .into_iter()
            .filter(|cell| cell.is_marked())
            .collect();
        let expected_marked_cells: Vec<Cell> = nums
            .iter()
            .copied()
            .filter(|&num| num == num_to_mark)
            .map(Cell::Marked)
            .collect();
        assert_eq!(marked_cells, expected_marked_cells);
    }

    #[test]
    fn gets_cell_by_row_and_column() {
        let board = Board::from(BOARD_STR);
        let cell_0_0 = board.get(0, 0);
        let cell_5_5 = board.get(4, 4);
        let cell_2_3 = board.get(2, 3);
        assert_eq!(cell_0_0, Some(&Cell::UnMarked(14)));
        assert_eq!(cell_5_5, Some(&Cell::UnMarked(7)));
        assert_eq!(cell_2_3, Some(&Cell::UnMarked(26)));
    }

    #[test]
    fn checks_if_row_is_marked() {
        let first_row: [u16; 5] = [14, 21, 17, 24, 4];
        let board = Board::from(BOARD_STR);
        assert!(!board.has_won());
        let board = first_row.iter().fold(board, |board, &num| board.mark(num));
        assert!(board.has_won());
    }

    #[test]
    fn checks_row_not_marked() {
        let almost_first_row: [u16; 4] = [14, 21, 17, 24];
        let board = Board::from(BOARD_STR);
        assert!(!board.has_won());
        let board = almost_first_row
            .iter()
            .fold(board, |board, &num| board.mark(num));
        assert!(!board.has_won());
    }

    #[test]
    fn checks_marked_column() {
        let first_column: [u16; 5] = [14, 10, 18, 22, 2];
        let board = Board::from(BOARD_STR);
        assert!(!board.has_won());
        let board = first_column
            .iter()
            .fold(board, |board, &num| board.mark(num));
        assert!(board.has_won());
    }
}
