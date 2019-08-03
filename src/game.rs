use crate::POINTS_TO_WIN;

pub fn setup_game(p1: &mut super::player::Player, p2: &mut super::player::Player) {
    p1.reset_score();
    p1.to_not();
    p1.reset_board();
    p2.reset_score();
    p2.to_not();
    p2.reset_board();
}


pub fn play_game(p1: &mut super::player::Player, p2: &mut super::player::Player) {
    while *p1.get_state() != super::State::Done 
        && *p2.get_state() != super::State::Done {
            super::turn::draw(p1);
            super::disp_board::disp(p1, p2); //Its like this because all the turn stuff takes 1 argumet and the disp_board stuff needs 2
            super::turn::play(p1);
            super::disp_board::disp(p1, p2);
            super::turn::choose_stand(p1);
            super::disp_board::disp(p1, p2); 
            
            super::turn::draw(p2);
            super::disp_board::disp(p1, p2);
            super::turn::play(p2);
            super::disp_board::disp(p1, p2); 
            super::turn::choose_stand(p2);
            super::disp_board::disp(p1, p2); 
    }
    while *p1.get_state() != super::State::Done {
            super::turn::draw(p1);
            super::disp_board::disp(p1, p2); //Its like this because all the turn stuff takes 1 argumet and the disp_board stuff needs 2
            super::turn::play(p1);
            super::disp_board::disp(p1, p2);
            super::turn::choose_stand(p1);
            super::disp_board::disp(p1, p2);  
    }
    while *p2.get_state() != super::State::Done {
            super::turn::draw(p2);
            super::disp_board::disp(p1, p2);
            super::turn::play(p2);
            super::disp_board::disp(p1, p2); 
            super::turn::choose_stand(p2);
            super::disp_board::disp(p1, p2);;
    }  
    find_winner(p1, p2);
}

pub fn find_winner(p1: &mut super::player::Player, p2: &mut super::player::Player) {
    let p1_score = p1.get_score();
    let p2_score = p2.get_score();

    if p1_score > POINTS_TO_WIN && p2_score > POINTS_TO_WIN {
        println!("Both players over");
    }
    else if p1_score > POINTS_TO_WIN {
        println!("{} is over {}, so {} wins!", p1.get_name(), POINTS_TO_WIN, p2.get_name());
        p2.add_win();
    }
    else if p2_score > POINTS_TO_WIN {
        println!("{} is over {}, so {} wins!", p2.get_name(), POINTS_TO_WIN, p1.get_name());
        p1.add_win();
    }
    else if p1_score < p2_score {
        println!("Both players have less than {} points, but {} has more points than {}, so {} wins!",
                 POINTS_TO_WIN, p2.get_name(), p1.get_name(), p2.get_name());
        p2.add_win();
    }
    else if p1_score > p2_score {
         println!("Both players have less than {} points, but {} has more points than {}, so {} wins!",
                 POINTS_TO_WIN, p1.get_name(), p2.get_name(), p1.get_name());
         p1.add_win();
    }
    else if p1_score == p2_score {
        println!("Both players had scores under {} and had the same score. Thus, no wins are awarded", POINTS_TO_WIN);
    }
    else {
        println!("Something went wrong with the logic in the 'find_winner' function of the game.rs doc");
    }
}
