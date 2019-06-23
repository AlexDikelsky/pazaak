pub struct Player {
    hand: Vec<super::card::Card>,
    board: [[super::card::Card; 3]; 3],
    score: i8,
    wins: u8,
    player_no: u8,
    state: State,
}

enum State {
    Draw,
    Stand,
    Wait,
}

impl Player {
    pub fn make_player() -> Player {
        Player { hand: super::hand::make_hand(), board: [[super::card::Card::empty_card(), super::card::Card::empty_card(),
                                                                            super::card::Card::empty_card()],
                                                        [super::card::Card::empty_card(), super::card::Card::empty_card(), 
                                                                            super::card::Card::empty_card()],
                                                        [super::card::Card::empty_card(), super::card::Card::empty_card(), 
                                                                            super::card::Card::empty_card()]],
                score: 0, wins: 0, player_no: 0, state: State::Draw}
    }
    pub fn make_ai() -> Player {
        Player { hand: super::hand::make_rand_hand(), board: [[super::card::Card::empty_card(), super::card::Card::empty_card(),
                                                                            super::card::Card::empty_card()],
                                                        [super::card::Card::empty_card(), super::card::Card::empty_card(), 
                                                                            super::card::Card::empty_card()],
                                                        [super::card::Card::empty_card(), super::card::Card::empty_card(), 
                                                                            super::card::Card::empty_card()]],
                score: 0, wins: 0, player_no: 0, state: State::Draw}
    }

    pub fn add_card(&mut self, new_card: &super::card::Card) {
        let mut i = 0;
        let mut placed = false;
        let mut j = 0;

        while i < 3 && !placed {
            while j < 3 && !placed {
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
    pub fn to_stand(&mut self) {
        self.state = State::Stand;
    }
    pub fn to_wait(&mut self) {
        self.state = State::Wait;
    }
    pub fn to_draw(&mut self) {
        self.state = State::Draw;
    }
    pub fn get_board(&self) -> &[[super::card::Card; 3]; 3] {
        &self.board
    }
    pub fn get_card(&self, card_num: usize) -> &super::card::Card {
        &self.hand[card_num]
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
        self.player_no
    }
}
