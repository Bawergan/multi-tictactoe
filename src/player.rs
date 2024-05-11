#[derive(Clone, Debug, PartialEq, Copy)]
pub enum PlayerType {
    Bot,
    Terminal,
    JankyBot,
    Deeper,
    Deeperer,
    Deeperest,
    Martin,
}
#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Player {
    pub id: usize,
    pub skin: char,
    pub p_type: PlayerType,
    pub score: usize,
}

impl Player {
    pub fn new(id: usize, skin: char) -> Self {
        Player {
            id,
            skin,
            p_type: PlayerType::Terminal,
            score: 0,
        }
    }
    pub fn new_custom(id: usize, skin: char, p_type: PlayerType) -> Self {
        Player {
            id,
            skin,
            p_type,
            score: 0,
        }
    }
    pub fn new_bot(id: usize, skin: char) -> Self {
        Player {
            id,
            skin,
            p_type: PlayerType::Bot,
            score: 0,
        }
    }
    pub fn new_dummy() -> Self {
        Player {
            id: 999,
            skin: ' ',
            p_type: PlayerType::Bot,
            score: 0,
        }
    }
}
