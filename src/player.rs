use std::collections::HashMap;

pub struct Player {
    hand: HashMap<u8, super::card::Card>,
    board: [[super::card::Card; 3]; 3],
    score: i8,
    wins: u8,
    side: u8,
}

impl Player {
    pub fn make_player() -> Player {
        Player { hand: super::hand::make_hand(), board: [[super::card::Card::empty_card(), super::card::Card::empty_card(),
                                                                            super::card::Card::empty_card()],
                                                        [super::card::Card::empty_card(), super::card::Card::empty_card(), 
                                                                            super::card::Card::empty_card()],
                                                        [super::card::Card::empty_card(), super::card::Card::empty_card(), 
                                                                            super::card::Card::empty_card()]],
                                                score: 0, wins: 0, side: 0 }
    }
    pub fn make_ai() -> Player {
        Player { hand: super::hand::make_hand(), board: [[super::card::Card::empty_card(), super::card::Card::empty_card(),
                                                                            super::card::Card::empty_card()],
                                                        [super::card::Card::empty_card(), super::card::Card::empty_card(), 
                                                                            super::card::Card::empty_card()],
                                                        [super::card::Card::empty_card(), super::card::Card::empty_card(), 
                                                                            super::card::Card::empty_card()]],
                score: 0, wins: 0, side: 0 }
    }
    
    /*fn empty_cards() -> [[super::card::Card;3];3] {
        let x = [[super::card::Card;3];3];
        for i in x {
            for j in i {
                x[i][j] = super::card::Card::empty_card();
            }
        }
        x
    }*/
    
    pub fn get_card(&self, card_num: u8) -> &super::card::Card {
        return &self.hand[&card_num];
    }
    pub fn change_score(&mut self, by: i8) {
        self.score += by;
    }
    pub fn get_score(&self) -> i8{
        self.score
    }
    pub fn add_win(&mut self) {
        self.wins += 1;
    }
    pub fn get_wins(&self) -> u8 {
        self.wins
    }
    pub fn get_side(&self) -> u8 {
        self.side
    }
}
