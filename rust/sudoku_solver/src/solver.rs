use crate::{board::Board, cell::Cell, display::Display};

pub struct SolverOptions {
    pub print_debug: bool,
    pub one_pencil_rem_cell: bool,
    pub one_pencil_rem_column: bool,
    pub one_pencil_rem_row: bool,
    pub one_pencil_rem_box: bool,
    pub pointing_pencil_column: bool,
    pub pointing_pencil_row: bool,
}

impl SolverOptions {
    pub fn all() -> Self {
        Self {
            print_debug: true,
            one_pencil_rem_cell: true,
            one_pencil_rem_column: true,
            one_pencil_rem_row: true,
            one_pencil_rem_box: true,
            pointing_pencil_column: true,
            pointing_pencil_row: true,
        }
    }
}

#[allow(clippy::needless_range_loop)]
pub fn is_solved(board: &Board) -> bool {
    let mut cols = [[0; 9]; 9];
    let mut rows = [[0; 9]; 9];
    let mut boxes = [[0; 9]; 9];

    for row in 0..9 {
        for col in 0..9 {
            let boxi = (row / 3) * 3 + (col / 3) * 3;
            match board.get_cell((col, row)) {
                Cell::Filled(n) => {
                    let n = *n as usize;
                    cols[col][n] += 1;
                    rows[row][n] += 1;
                    boxes[boxi][n] += 1;
                }
                _ => return false,
            }
        }
    }

    cols.iter()
        .chain(rows.iter())
        .chain(boxes.iter())
        .all(|e| e.iter().all(|c| *c == 1))
}

/// Returns whether the board was successfully solved
pub fn attempt_solve(board: &mut Board, options: &SolverOptions) -> bool {
    // Initialize pencil marks (possibilities)

    // Rows
    for row in 0..9 {
        let mut nums = [true; 9];
        for col in 0..9 {
            if let Cell::Filled(n) = board.get_cell((col, row)) {
                nums[*n as usize - 1] = false;
            }
        }
        for col in 0..9 {
            if let Cell::Empty(p) = board.get_mut_cell((col, row)) {
                p.possible_numbers = nums;
            }
        }
    }
    // Cols
    for col in 0..9 {
        let mut nums = [true; 9];
        for row in 0..9 {
            if let Cell::Filled(n) = board.get_cell((col, row)) {
                nums[*n as usize - 1] = false;
            }
        }
        for row in 0..9 {
            if let Cell::Empty(p) = board.get_mut_cell((col, row)) {
                for (i, b) in p.possible_numbers.iter_mut().enumerate() {
                    *b = *b && nums[i];
                }
            }
        }
    }
    // Boxes
    for brow in 0..3 {
        for bcol in 0..3 {
            let mut nums = [true; 9];
            for row in brow * 3..(brow + 1) * 3 {
                for col in bcol * 3..(bcol + 1) * 3 {
                    if let Cell::Filled(n) = board.get_cell((col, row)) {
                        nums[*n as usize - 1] = false;
                    }
                }
            }
            for row in brow * 3..(brow + 1) * 3 {
                for col in bcol * 3..(bcol + 1) * 3 {
                    if let Cell::Empty(p) = board.get_mut_cell((col, row)) {
                        for (i, b) in p.possible_numbers.iter_mut().enumerate() {
                            *b = *b && nums[i];
                        }
                    }
                }
            }
        }
    }

    let mut stepped = true;
    while stepped {
        if options.print_debug {
            board.display_full();
        }
        stepped = false;

        // Pointing pencil
        if options.pointing_pencil_column {
            strategies::pointing_pencil_column(board, options);
        }
        if options.pointing_pencil_row {
            strategies::pointing_pencil_row(board, options);
        }

        // One pencil remaining
        if options.one_pencil_rem_cell {
            stepped |= strategies::one_pencil_rem_cell(board);
        }
        if options.one_pencil_rem_column {
            stepped |= strategies::one_pencil_rem_column(board);
        }
        if options.one_pencil_rem_row {
            stepped |= strategies::one_pencil_rem_row(board);
        }
        if options.one_pencil_rem_box {
            stepped |= strategies::one_pencil_rem_box(board);
        }
    }

    is_solved(board)
}

pub mod strategies {
    use crate::{
        board::Board,
        cell::{Cell, CellNumber},
        pos::PosInt,
    };

    use super::SolverOptions;

    pub fn one_pencil_rem_cell(board: &mut Board) -> bool {
        let mut stepped = false;

        for row in 0..9 {
            for col in 0..9 {
                let c = board.get_mut_cell((col, row));
                if let Cell::Empty(p) = c
                    && p.possible_numbers.iter().filter(|b| **b).count() == 1
                {
                    let n = p
                        .possible_numbers
                        .iter()
                        .enumerate()
                        .find(|(_, b)| **b)
                        .unwrap()
                        .0 as CellNumber
                        + 1;
                    println!("placed a {n} by 1-pencil-rem-cell at {col},{row}");
                    board.place_number((col, row), n);
                    stepped = true;
                }
            }
        }

        stepped
    }

    pub fn one_pencil_rem_column(board: &mut Board) -> bool {
        let mut stepped = false;
        for col in 0..9 {
            let mut nums = [(None, false); 9];
            for row in 0..9 {
                if let Cell::Empty(p) = board.get_mut_cell((col, row)) {
                    p.possible_numbers.iter().enumerate().for_each(|(i, b)| {
                        if !*b {
                            return;
                        }
                        if nums[i].0.is_none() {
                            nums[i].0 = Some(row);
                            nums[i].1 = true;
                        } else {
                            nums[i].1 = false;
                        }
                    });
                }
            }
            for (n, (o, _)) in nums
                .iter()
                .enumerate()
                .filter(|(_, (o, b))| o.is_some() && *b)
            {
                let row = o.unwrap() as PosInt;
                let n = n as CellNumber + 1;
                println!("placed a {n} by 1-pencil-rem-row at {col},{row}");
                board.place_number((col, row), n);
                stepped = true;
            }
        }

        stepped
    }

    pub fn one_pencil_rem_row(board: &mut Board) -> bool {
        let mut stepped = false;

        for row in 0..9 {
            let mut nums = [(None, false); 9];
            for col in 0..9 {
                if let Cell::Empty(p) = board.get_mut_cell((col, row)) {
                    p.possible_numbers.iter().enumerate().for_each(|(i, b)| {
                        if !*b {
                            return;
                        }
                        if nums[i].0.is_none() {
                            nums[i].0 = Some(col);
                            nums[i].1 = true;
                        } else {
                            nums[i].1 = false;
                        }
                    });
                }
            }
            for (n, (o, _)) in nums
                .iter()
                .enumerate()
                .filter(|(_, (o, b))| o.is_some() && *b)
            {
                let col = o.unwrap() as PosInt;
                let n = n as CellNumber + 1;
                println!("placed a {n} by 1-pencil-rem-row at {col},{row}");
                board.place_number((col, row), n);
                stepped = true;
            }
        }

        stepped
    }

    pub fn one_pencil_rem_box(board: &mut Board) -> bool {
        let mut stepped = false;
        for brow in 0..3 {
            for bcol in 0..3 {
                let mut nums = [(None, false); 9];
                for irow in 0..3 {
                    for icol in 0..3 {
                        let (col, row) = (bcol * 3 + icol, brow * 3 + irow);
                        if let Cell::Empty(p) = board.get_cell((col, row)) {
                            p.possible_numbers.iter().enumerate().for_each(|(n, b)| {
                                if !*b {
                                    return;
                                }
                                if nums[n].0.is_none() {
                                    nums[n].0 = Some((col, row));
                                    nums[n].1 = true;
                                } else {
                                    nums[n].1 = false;
                                }
                            });
                        }
                    }
                }
                for (n, (o, _)) in nums
                    .iter()
                    .enumerate()
                    .filter(|(_, (o, b))| o.is_some() && *b)
                {
                    let (col, row) = o.unwrap();
                    let n = n as CellNumber + 1;
                    println!("placed a {n} by 1-pencil-rem-box at {col},{row}");
                    board.place_number((col, row), n);
                    stepped = true;
                }
            }
        }

        stepped
    }

    pub fn pointing_pencil_column(board: &mut Board, options: &SolverOptions) -> bool {
        let mut stepped = false;
        for brow in 0..3 {
            for bcol in 0..3 {
                let row1 = brow * 3;
                let row2 = (brow + 1) * 3;
                for (icol1, icol2, icolo) in [(0, 1, 2), (0, 2, 1), (1, 2, 0)] {
                    let (col1, col2, colo) = (bcol * 3 + icol1, bcol * 3 + icol2, bcol * 3 + icolo);
                    for n in 1..=9 {
                        if !board.has_pencil_in_range((col1, row1), (col1 + 1, row2), n)
                            && !board.has_pencil_in_range((col2, row1), (col2 + 1, row2), n)
                            && board.has_pencil_in_range((colo, row1), (colo + 1, row2), n)
                        {
                            if options.print_debug {
                                println!(
                                    "erased pencil {n} by pointing-pencil-col ({col1},{col2}) at box {bcol},{brow}"
                                );
                            }
                            board.remove_pencil_in_range((colo, 0), (colo + 1, row1), n);
                            board.remove_pencil_in_range((colo, row2), (colo + 1, 9), n);
                            stepped = true;
                        }
                    }
                }
            }
        }
        stepped
    }

    pub fn pointing_pencil_row(board: &mut Board, options: &SolverOptions) -> bool {
        let mut stepped = false;

        for brow in 0..3 {
            for bcol in 0..3 {
                let col1 = bcol * 3;
                let col2 = (bcol + 1) * 3;
                for (irow1, irow2, irowo) in [(0, 1, 2), (0, 2, 1), (1, 2, 0)] {
                    let (row1, row2, rowo) = (brow * 3 + irow1, brow * 3 + irow2, brow * 3 + irowo);
                    for n in 1..=9 {
                        if !board.has_pencil_in_range((col1, row1), (col2, row1 + 1), n)
                            && !board.has_pencil_in_range((col1, row2), (col2, row2 + 1), n)
                            && board.has_pencil_in_range((col1, rowo), (col2, rowo + 1), n)
                        {
                            if options.print_debug {
                                println!(
                                    "erased pencil {n} by pointing-pencil-row ({row1},{row2}) at box {bcol},{brow}"
                                );
                            }
                            board.remove_pencil_in_range((0, rowo), (col1, rowo + 1), n);
                            board.remove_pencil_in_range((col2, rowo), (9, rowo + 1), n);
                            stepped = true;
                        }
                    }
                }
            }
        }
        stepped
    }
}

