use crate::GAMES_TO_WIN;

pub fn play_series(p1: &mut super::player::Player, p2: &mut super::player::Player) {
    while p1.get_wins() < GAMES_TO_WIN && p2.get_wins() < GAMES_TO_WIN {
        super::game::setup_game(p1, p2);        
        super::game::play_game(p1, p2);
    }
    if p1.get_wins() >= 3 {
        println!("{} WON!!!!", p1.get_name());
    }
    if p2.get_wins() >= 3 {
        println!("{} WON!!!!", p2.get_name());
    }
}

