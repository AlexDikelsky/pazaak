pub fn disp(p1: &super::player::Player, p2: &super::player::Player) {
    let p1_name = "Player 1";
    let p2_name = "Player 2";
 
 
    let top = "|‾‾‾‾|";
    let bot = "|____|";
    let l_side = "| ";
    let r_side = " |";
    let eight = "        ";
    let ten = "          ";
    let one = " ";
 
    println!("{}{}{}{}{}{}{}{}{}", p1_name, eight, -2, ten, "000-000", ten, 12, eight, p2_name);
    println!("{}{}{}{}{}{}{}{}{}{}{}{}", eight, top, one, top, one, top, eight, top, one, top, one, eight);
    println!("{}{}", eight, l_side);
}
