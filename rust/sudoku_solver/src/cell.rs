use crate::pencil::Pencil;

pub type CellNumber = u8;

#[derive(Clone, Debug)]
pub enum Cell {
    Empty(Pencil),
    Filled(CellNumber),
}

impl Cell {
    pub const fn empty() -> Self {
        Self::Empty(Pencil::empty())
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty(_))
    }

    pub fn is_filled(&self) -> bool {
        matches!(self, Cell::Filled(_))
    }

    pub fn get_mut_empty(&mut self) -> &mut Pencil {
        match self {
            Self::Filled(_) => panic!("Called get_mut_empty on non-empty cell"),
            Self::Empty(p) => p,
        }
    }

    pub fn has_pencil(&self, n: CellNumber) -> bool {
        if let Cell::Empty(p) = self
            && p.possible_numbers[n as usize - 1]
        {
            true
        } else {
            false
        }
    }

    pub fn set_pencil(&mut self, n: CellNumber, state: bool) {
        if let Cell::Empty(p) = self {
            p.possible_numbers[n as usize - 1] = state;
        }
    }

    pub fn set_fill(&mut self, n: CellNumber) {
        *self = Cell::Filled(n);
    }
}

impl ::std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty(_) => write!(f, "_"),
            Self::Filled(num) => write!(f, "{num}"),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::Empty(Default::default())
    }
}
