use crate::player::Player;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Filed(Player),
}