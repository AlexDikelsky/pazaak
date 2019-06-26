pub fn play_series(p1: &mut super::player::Player, p2: &mut super::player::Player) {
    while p1.get_wins() < 3 && p2.get_wins() < 3 {
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

