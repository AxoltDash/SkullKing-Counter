#[derive(Clone)]
pub struct Player {
    name: String,
    points: i32,
}

fn build_new_player(name: String) -> Player {
    Player {
        name,
        points: 0,
    }
}

pub fn build_players_list(names_list: Vec<String>) -> Vec<Player> {
    let mut players_list = Vec::new();
    for name in names_list {
       players_list.push(build_new_player(name.clone())); 
    }
    players_list
}

pub fn add_points(p: &mut Player, x: i32) {
    p.points += x
}

pub fn get_name(p: &Player) -> String {
    p.name.clone()
}

pub fn get_points(p: &Player) -> i32 {
    p.points
}


pub fn sort_players_desc(players: &Vec<Player>) -> Vec<Player> {
    let mut sorted_players = players.clone();
    sorted_players.sort_by(|a, b| get_points(b).cmp(&get_points(a)));
    sorted_players
}

