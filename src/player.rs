#[derive(Clone, Debug, PartialEq, Copy)]
pub enum PlayerType {
    Bot,
    Terminal,
}
#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Player {
    pub id: usize,
    pub skin: char,
    pub p_type: PlayerType,
}

impl Player {
    pub fn new(id: usize, skin: char) -> Self {
        Player {
            id,
            skin,
            p_type: PlayerType::Terminal,
        }
    }
    pub fn new_bot(id: usize, skin: char) -> Self {
        Player {
            id,
            skin,
            p_type: PlayerType::Bot,
        }
    }
}
