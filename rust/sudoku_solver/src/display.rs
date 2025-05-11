use crate::{board::Board, cell::Cell};

pub trait Display {
    fn display_basic(&self);

    fn display_full(&self);
}

impl Display for Board {
    fn display_basic(&self) {
        println!();
        for row in 0..9 {
            for col in 0..9 {
                print!("{}", self.get_cell((col, row)));
            }
            println!();
        }
    }

    fn display_full(&self) {
        println!("\n+---+---+---+---+---+---+---+---+---+");
        for row in 0..9 {
            // Top line
            for col in 0..9 {
                print!("|");
                let cell = self.get_cell((col, row));
                match cell {
                    Cell::Filled(_) => print!(". ."),
                    Cell::Empty(p) => print!(
                        "{}{}{}",
                        if p.possible_numbers[0] { '1' } else { '_' },
                        if p.possible_numbers[1] { '2' } else { '_' },
                        if p.possible_numbers[2] { '3' } else { '_' }
                    ),
                }
            }
            println!("|");
            // Middle line
            for col in 0..9 {
                print!("|");
                let cell = self.get_cell((col, row));
                match cell {
                    Cell::Filled(n) => print!(" {n} "),
                    Cell::Empty(p) => print!(
                        "{}{}{}",
                        if p.possible_numbers[3] { '4' } else { '_' },
                        if p.possible_numbers[4] { '5' } else { '_' },
                        if p.possible_numbers[5] { '6' } else { '_' }
                    ),
                }
            }
            println!("|");
            // Bottom line
            for col in 0..9 {
                print!("|");
                let cell = self.get_cell((col, row));
                match cell {
                    Cell::Filled(_) => print!(". ."),
                    Cell::Empty(p) => print!(
                        "{}{}{}",
                        if p.possible_numbers[6] { '7' } else { '_' },
                        if p.possible_numbers[7] { '8' } else { '_' },
                        if p.possible_numbers[8] { '9' } else { '_' }
                    ),
                }
            }
            println!("|\n+---+---+---+---+---+---+---+---+---+");
        }
    }
}
