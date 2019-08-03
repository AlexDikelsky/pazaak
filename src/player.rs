use crate::BOARD_X;
use crate::BOARD_Y;

pub struct Player {
    hand: Vec<super::card::Card>,
    board: [[super::card::Card; BOARD_X]; BOARD_Y],
    score: i8,
    wins: u8,
    state: State,
    name: String,
}

#[derive(PartialEq)]
pub enum State {
    NotDone,
    Done,
}

impl Player {
    pub fn make_player(n: &str) -> Player {
        Player { hand: super::hand::make_hand(), board: [[super::card::Card::empty_card(), super::card::Card::empty_card(),
                                                                            super::card::Card::empty_card()],
                                                        [super::card::Card::empty_card(), super::card::Card::empty_card(), 
                                                                            super::card::Card::empty_card()],
                                                        [super::card::Card::empty_card(), super::card::Card::empty_card(), 
                                                                            super::card::Card::empty_card()]],
                score: 0, wins: 0, state: State::NotDone, name: n.to_string()}
    }
    pub fn make_ai(n: &str) -> Player {
        Player { hand: super::hand::make_rand_hand(), board: [[super::card::Card::empty_card(), super::card::Card::empty_card(),
                                                                            super::card::Card::empty_card()],
                                                        [super::card::Card::empty_card(), super::card::Card::empty_card(), 
                                                                            super::card::Card::empty_card()],
                                                        [super::card::Card::empty_card(), super::card::Card::empty_card(), 
                                                                            super::card::Card::empty_card()]],
                score: 0, wins: 0, state: State::NotDone, name: n.to_string()} 
    }

    pub fn add_card(&mut self, new_card: &super::card::Card) {
        let mut i = 0;
        let mut j = 0;
        let mut placed = false;

        while i < BOARD_Y && !placed {
            j = 0;
            while j < BOARD_X && !placed {
                if &self.board[i][j].get_value() == &0 {
                    placed = true;
                }
            j += 1;
            }
        i += 1;
        }
        self.score += new_card.get_value();
        self.board[i-1][j-1] = super::card::Card::make_card(new_card.get_value());
    }
    pub fn to_not(&mut self) {
        self.state = State::NotDone;
    }
    pub fn to_done(&mut self) {
        self.state = State::Done;
    }
    pub fn get_state(&self) -> &State {
        &self.state
    }
    pub fn get_board(&self) -> &[[super::card::Card; BOARD_X]; BOARD_Y] {
        &self.board
    }
    pub fn get_value_in_board(&self, i: usize, j: usize) -> &super::card::Card {
        &self.board[i][j]
    }
    pub fn get_card(&self, card_num:usize) -> &super::card::Card {
        &self.hand[card_num]
    }
    pub fn use_from_hand(&mut self, card_num: usize) -> i8 {
        let x = &self.hand[card_num].get_value();
        &self.hand[card_num].mark_used();
        *x    
    }
    pub fn get_score(&self) -> i8{
        self.score
    }
    pub fn reset_score(&mut self) {
        self.score = 0;
    }
    pub fn reset_board(&mut self) {
        self.board = [[super::card::Card::empty_card(), super::card::Card::empty_card(), super::card::Card::empty_card()],
                     [super::card::Card::empty_card(), super::card::Card::empty_card(), super::card::Card::empty_card()],
                     [super::card::Card::empty_card(), super::card::Card::empty_card(), super::card::Card::empty_card()]]
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn add_win(&mut self) {
        self.wins += 1;
    }
    pub fn get_wins(&self) -> u8 {
        self.wins
    }
}
