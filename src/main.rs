mod text_printer;
mod counter;
mod player;

struct Points(u8,u8,i32);

fn main() {
    text_printer::start();
    // text_printer::ron_ron_ron();
    loop {
        let strings_players = text_printer::obtain_players();
        let mut list = player::build_players_list(strings_players);
        for round in 1..11 {
            text_printer::ron_ron_ron();
            for p in &mut list {
                let points_this_round = text_printer::obtain_points(player::get_name(p));
                let calculated_points = counter::calculate_points(points_this_round, round);
                player::add_points(p, calculated_points);
            }
            text_printer::stats(&list, round);
        }
        text_printer::winner(&list);
        let flag = text_printer::play_again();
        if !flag { break; }
    }
}
