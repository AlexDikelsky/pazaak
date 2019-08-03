use crate::POINTS_TO_WIN;

pub fn draw(player: &mut super::player::Player) -> &mut super::player::Player {
    let card = super::card::Card::make_draw_card();
    &player.add_card(&card);
    println!("{} drew a {}", player.get_name(), card.get_value());
    player
}

pub fn play(player: &mut super::player::Player) -> &mut super::player::Player {
    println!("{}, Pick a card to play. Else, type 0", player.get_name());
    let choice = super::read::read_bounded_u8();
    if choice == 0 {}
    else {
        let card_value = &player.use_from_hand((choice-1) as usize);
        println!("Value = {}", card_value);
        player.add_card(&super::card::Card::make_card(*card_value));
    }
    player
}
pub fn choose_stand(player: &mut super::player::Player) -> &mut super::player::Player {
    println!("{}, to stand, type 1. To not stand, type anything", player.get_name());
    let x = super::read::read_i8();
    if x == 1 || player.get_score() > POINTS_TO_WIN  {
        player.to_done(); 
    }
    player
}
