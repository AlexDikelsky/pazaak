pub fn draw(player: &mut super::player::Player) -> () {
    let card = super::card::Card::make_draw_card();
    &player.add_card(&card);
    //play(player);
}

pub fn play(player: &mut super::player::Player) -> () {
    //Adds a card. Type 0 to not play a card
    println!("Pick a card to play. Else, type 0");
    let choice = super::read::read_i8();
    if choice > 0 {
        
    }
    else {
        //player.add_card(player.get_card((choice+1) as u8));
        let card_value = &player.get_card((choice+1) as usize).get_value();
        player.add_card(&super::card::Card::make_card(*card_value));
    }
    /*choose_stand(player)*/;
}
/*
pub fn choose_stand(mut player: super::player::Player) -> () {
    println!("Type 0 to stand, else type 1");
    let x = super::read::read_i8();
    if x == 0 || player.get_score() > 20 {
        player.to_stand();
    }
    else {
        player.to_wait();
    }
}
*/
