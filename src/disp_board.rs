use crate::BOARD_X;
use crate::BOARD_Y;

static TOP:&str = "|‾‾‾‾|";
static BOT:&str = "|____|";
static ONE:&str = " ";
static TWO:&str = "  ";
static FIVE:&str = "     ";
static EIGHT:&str = "        ";
static TEN:&str = "          ";
static SEVEN:&str = "       ";
static ELEVEN:&str = "           ";
static FIFTEEN:&str = "               ";

pub fn disp(p1: &super::player::Player, p2: &super::player::Player) {
    let mut p1_board = p1.get_board().iter();
    let mut p2_board = p2.get_board().iter();

    super::clear::clear();

    print_header(p1, p2);
    println!();
    for _i in 0..BOARD_Y {
        print_row(&mut p1_board.next().unwrap(), &mut p2_board.next().unwrap());
    }
    print_hand_header(p1, p2); 
    print_hand(p1, p2);
    println!();
}

fn print_header(p1: &super::player::Player, p2: &super::player::Player) {
    println!("{}{}{}{}{}{}{}{}{}", p1.get_name(), EIGHT, disp_score(p1.get_score()), TEN,
            match_score(p1, p2), TEN, disp_score(p2.get_score()), EIGHT, p2.get_name());
}

fn print_row(p1_row: &[super::card::Card; BOARD_X], p2_row: &[super::card::Card; BOARD_X]) {
    println!("{}{}{}{}{}{}{}{}{}{}{}{}", EIGHT, TOP, ONE, TOP, ONE, TOP, SEVEN, TOP, ONE, TOP, ONE, TOP);
    println!("{}{}{}{}{}{}{}{}{}{}{}{}", EIGHT, 
                                          p1_row[0].mid_line(), ONE, p1_row[1].mid_line(), ONE, p1_row[2].mid_line(), 
                                          SEVEN,
                                          p2_row[0].mid_line(), ONE, p2_row[1].mid_line(), ONE, p2_row[2].mid_line(), 
                                        );
    println!("{}{}{}{}{}{}{}{}{}{}{}{}", EIGHT, BOT, ONE, BOT, ONE, BOT, SEVEN, BOT, ONE, BOT, ONE, BOT);
    println!();
}
fn print_hand_header(p1: &super::player::Player, p2: &super::player::Player) {
    println!("{}{}{}{}{}{}", ELEVEN, p1.get_name(), " Hand", FIFTEEN, p2.get_name(), " Hand");
}
fn print_hand(p1: &super::player::Player, p2: &super::player::Player) {
    println!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", TWO, TOP, ONE, TOP, ONE, TOP, ONE, TOP, FIVE, TOP, ONE, TOP, ONE, TOP, ONE, TOP);
    println!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", TWO, 
                                                  p1.get_card(0).mid_line(), ONE, p1.get_card(1).mid_line(), ONE,
                                                    p1.get_card(2).mid_line(), ONE, p1.get_card(3).mid_line(),
                                                 FIVE, 
                                                  p2.get_card(0).mid_line(), ONE, p2.get_card(1).mid_line(), ONE,
                                                    p2.get_card(2).mid_line(), ONE, p2.get_card(3).mid_line()
                                                );

    println!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", TWO, BOT, ONE, BOT, ONE, BOT, ONE, BOT, FIVE, BOT, ONE, BOT, ONE, BOT, ONE, BOT);
}

fn disp_score(x: i8) -> String {
    if x < 10 && x >= 0 {
        String::from(" ".to_string() + &x.to_string())
    }
    else if x >= 10 {
        x.to_string()
    }
    else {
        x.to_string()
    }
}
fn match_score(p1: &super::player::Player, p2: &super::player::Player) -> String {
    String::from(p_score_to_str(p1).to_string() + "-" + &p_score_to_str(p2))
}
fn p_score_to_str(p: &super::player::Player) -> String{
    match p.get_wins() {
        0 => "000".to_string(),
        1 => "X00".to_string(),
        2 => "XX0".to_string(),
        3 => "XXX".to_string(),
        _ => format!("{:03}", p.get_wins()),
    }
}
