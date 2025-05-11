#[derive(Clone, Debug)]
pub struct Pencil {
    pub possible_numbers: [bool; 9],
}

impl Pencil {
    pub const fn empty() -> Self {
        Self {
            possible_numbers: [false; 9],
        }
    }
}

impl Default for Pencil {
    fn default() -> Self {
        Self::empty()
    }
}
