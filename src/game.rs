pub fn setup_game(p1: &mut super::player::Player, p2: &mut super::player::Player) {
    let num_decisive_games = p1.get_wins() + p2.get_wins();
    
    p1.reset_score();
    p2.reset_score();

    if num_decisive_games % 2 == 0 {
        p1.to_my();
        p2.to_your();
        play_game(p1, p2);
    }
    else {
        p1.to_your();
        p2.to_my();
        play_game(p2, p1);
    }
}


pub fn play_game(p1: &mut super::player::Player, p2: &mut super::player::Player) {
    while *p1.get_state() != super::State::Done 
        && *p2.get_state() != super::State::Done {
            super::turn::draw(p1);
            super::disp_board::disp(p1, p2);
            super::turn::draw(p2);
            super::disp_board::disp(p1, p2);
        }
    while *p1.get_state() != super::State::Done {
        super::turn::draw(p1);
        super::disp_board::disp(p1, p2);
   
    }
    while *p2.get_state() != super::State::Done {
        super::turn::draw(p2);
        super::disp_board::disp(p1, p2);
    }  
    find_winner(p1, p2);
}

pub fn find_winner(p1: &mut super::player::Player, p2: &mut super::player::Player) {
    let p1_score = p1.get_score();
    let p2_score = p2.get_score();

    if p1_score > 20 && p2_score > 20 {
        println!("Both players over");
    }
    else if p1_score > 20 {
        println!("{} is over 20, so {} wins!", p1.get_name(), p2.get_name());
        p2.add_win();
    }
    else if p2_score > 20 {
        println!("{} is over 20, so {} wins!", p2.get_name(), p1.get_name());
        p1.add_win();
    }
    else if p1_score < p2_score {
        println!("Both players have less than 20 points, but {} has more points than {}, so {} wins!",
                 p2.get_name(), p1.get_name(), p2.get_name());
        p2.add_win();
    }
    else if p1_score > p2_score {
         println!("Both players have less than 20 points, but {} has more points than {}, so {} wins!",
                 p1.get_name(), p2.get_name(), p1.get_name());
         p1.add_win();
    }
    else if p1_score == p2_score {
        println!("Both players had scores under 20 and had the same score. Thus, no wins are awarded");
    }
    else {
        println!("Something went wrong with the logic in the 'find_winner' function of the game.rs doc");
    }
}
