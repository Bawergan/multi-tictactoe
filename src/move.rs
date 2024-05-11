use crate::player::Player;

#[derive(Clone, Debug, Copy)]
pub struct Move {
    pub coord: (usize, usize),
    pub player: Player,
}
impl Move {
    pub fn new(coord: (usize, usize), player: Player) -> Self {
        Move { coord, player }
    }
}
