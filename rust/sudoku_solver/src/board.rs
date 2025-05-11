use crate::{
    cell::{Cell, CellNumber},
    pos::{PosInt, ToIndex, ToPos, pos_to_box},
};

pub struct Board {
    pub grid: [Cell; 81],
}

impl Board {
    pub fn get_cell(&self, p: impl ToPos) -> &Cell {
        &self.grid[p.to_idx()]
    }

    pub fn get_mut_cell(&mut self, p: impl ToPos) -> &mut Cell {
        &mut self.grid[p.to_idx()]
    }

    pub const fn empty() -> Self {
        Self {
            grid: [const { Cell::empty() }; 81],
        }
    }

    pub fn place_number(&mut self, pos: impl ToPos, n: CellNumber) {
        let (col, row) = pos.to_pos();
        if matches!(self.get_cell((col, row)), Cell::Empty(_)) {
            self.get_mut_cell((col, row)).set_fill(n);

            // Invalidate pencil marks on the same column, row, and box
            // Row
            for col2 in 0..9 {
                self.get_mut_cell((col2, row)).set_pencil(n, false);
            }
            // Col
            for row2 in 0..9 {
                self.get_mut_cell((col, row2)).set_pencil(n, false);
            }
            // Box
            let (bcol, brow) = pos_to_box((col, row));
            for row2 in brow {
                for col2 in bcol.clone() {
                    self.get_mut_cell((col2, row2)).set_pencil(n, false);
                }
            }
        }
    }

    /// 2d range,
    /// pos1..pos2
    /// [pos1,pos2)
    pub fn remove_pencil_in_range(&mut self, pos1: impl ToPos, pos2: impl ToPos, n: CellNumber) {
        let (col1, row1) = pos1.to_pos();
        let (col2, row2) = pos2.to_pos();
        for row in row1..row2 {
            for col in col1..col2 {
                self.get_mut_cell((col, row)).set_pencil(n, false);
            }
        }
    }

    pub fn has_pencil_in_range(&self, pos1: impl ToPos, pos2: impl ToPos, n: CellNumber) -> bool {
        let (col1, row1) = pos1.to_pos();
        let (col2, row2) = pos2.to_pos();
        for row in row1..row2 {
            for col in col1..col2 {
                if self.get_cell((col, row)).has_pencil(n) {
                    return true;
                }
            }
        }
        false
    }

    pub fn all_have_pencil_in_range(
        &self,
        pos1: impl ToPos,
        pos2: impl ToPos,
        n: CellNumber,
    ) -> bool {
        let (col1, row1) = pos1.to_pos();
        let (col2, row2) = pos2.to_pos();
        let mut all_have = true;
        for row in row1..row2 {
            for col in col1..col2 {
                all_have = all_have && self.get_cell((col, row)).has_pencil(n);
            }
        }
        all_have
    }

    pub fn iter_grid(&self) -> GridIter<'_> {
        GridIter::from_board(self)
    }
}

pub struct GridIter<'a> {
    col: PosInt,
    row: PosInt,
    board: &'a Board,
}

impl<'i> GridIter<'i> {
    pub fn from_board<'a: 'i>(board: &'a Board) -> Self {
        Self {
            col: 0,
            row: 0,
            board,
        }
    }
}

impl<'i> Iterator for GridIter<'i> {
    type Item = (PosInt, PosInt, &'i Cell);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == 8 && self.col == 8 {
            return None;
        }
        let ret = self.board.get_cell((self.col, self.row));
        if self.col == 8 {
            self.col = 0;
            self.row += 1;
        }
        Some((self.col, self.row, ret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_pencil_in_range() {
        let mut board = Board::empty();
        board.get_mut_cell((0, 0)).set_pencil(1, true);
        board.get_mut_cell((1, 1)).set_pencil(2, true);
        board.get_mut_cell((2, 2)).set_pencil(3, true);

        assert!(board.has_pencil_in_range((0, 0), (1, 1), 1));
        assert!(!board.has_pencil_in_range((0, 0), (1, 1), 2));
    }
}
