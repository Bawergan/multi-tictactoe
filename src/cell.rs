#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Filed(usize),
}