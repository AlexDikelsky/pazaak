use crate::POINTS_TO_WIN;

pub fn draw(player: &mut super::player::Player) -> () {
    let card = super::card::Card::make_draw_card();
    &player.add_card(&card);
    println!("{} drew a {}", player.get_name(), card.get_value());
    play(player);
}

pub fn play(player: &mut super::player::Player) -> () {
    //Adds a card. Type 0 to not play a card
    println!("Pick a card to play. Else, type 0");
    let choice = super::read::read_bounded_u8();
    if choice == 0 {}
    else {
        let card_value = &player.get_card((choice-1) as usize).get_value();
        player.add_card(&super::card::Card::make_card(*card_value));
    }
    choose_stand(player);
}
pub fn choose_stand(player: &mut super::player::Player) -> () {
    println!("To stand, type 1. To not stand, type anything");
    let x = super::read::read_i8();
    if x == 1 || player.get_score() > POINTS_TO_WIN  {
        player.to_done(); 
    }
    else {
        player.to_your();
    }
}
