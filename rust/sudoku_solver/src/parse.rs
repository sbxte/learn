use crate::{board::Board, cell::Cell};

pub fn parse_str_1(input: &str) -> Result<Board, ParseError> {
    let mut board = Board::empty();
    let mut i = 0;
    for line in input.lines() {
        for char in line.chars() {
            match char {
                c if (b'1'..=b'9').contains(&(c as u8)) => {
                    board.grid[i] = Cell::Filled((c as u8) - b'0')
                }
                c if c.is_whitespace() => continue,
                _ => board.grid[i] = Default::default(),
            };
            i += 1;
        }
    }
    Ok(board)
}

#[derive(Debug)]
pub enum ParseError {}
