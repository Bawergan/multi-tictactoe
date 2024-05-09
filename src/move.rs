use crate::player::Player;

#[derive(Clone, Debug, Copy)]
pub struct Move {
    pub coord: (usize, usize),
    pub player: Player,
    id: usize,
}
impl Move {
    pub fn new(coord: (usize, usize), player: Player, id: usize) -> Self {
        Move { coord, player, id }
    }
}
