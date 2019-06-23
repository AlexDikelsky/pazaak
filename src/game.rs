pub fn setup_game(p1: &mut super::player::Player, p2: &mut super::player::Player) {
    let num_decisive_games = p1.get_wins() + p2.get_wins();
    if num_decisive_games % 2 == 0 {
        p1.to_my();
        p2.to_your();
    }
    else {
        p1.to_your();
        p2.to_my();
    }
}


pub fn play_turns(p1: &mut super::player::Player, p2: &mut super::player::Player) {
    while *p1.get_state() != super::State::Done 
        && *p2.get_state() != super::State::Done {
            super::turn::draw(p1);
            super::turn::draw(p2);
        }

}
