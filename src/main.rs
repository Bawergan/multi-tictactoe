use crate::player::Player;
use lobby::Lobby;

mod board;
mod bot;
mod cell;
mod lobby;
mod r#move;
mod player;
mod utils;

fn main() {
    println!("type help to list all commands");
    let players = vec![
        // Player::new_custom(1, 'M', player::PlayerType::Martin),
        Player::new(1, 'X'),
        Player::new_bot(2, 'O'),
    ];

    let mut lobby: Lobby = Lobby::new();
    lobby.add_players(players);

    lobby.run()
}
