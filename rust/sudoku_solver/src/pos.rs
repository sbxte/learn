use std::ops::Range;

pub type PosInt = usize;

pub trait ToPos {
    fn to_pos(self) -> (PosInt, PosInt);
}

impl ToPos for (PosInt, PosInt) {
    fn to_pos(self) -> (PosInt, PosInt) {
        self
    }
}

impl ToPos for [usize; 2] {
    fn to_pos(self) -> (PosInt, PosInt) {
        (self[0], self[1])
    }
}

pub trait ToIndex {
    fn to_idx(self) -> usize;
}

impl<T> ToIndex for T
where
    T: ToPos,
{
    fn to_idx(self) -> usize {
        let p = self.to_pos();
        p.1 * 9 + p.0
    }
}

impl ToIndex for usize {
    fn to_idx(self) -> usize {
        self
    }
}

pub fn pos_to_box(pos: impl ToPos) -> (Range<PosInt>, Range<PosInt>) {
    let (col, row) = pos.to_pos();
    let bcol = col / 3;
    let brow = row / 3;
    ((bcol * 3..(bcol + 1) * 3), (brow * 3..(brow + 1) * 3))
}
