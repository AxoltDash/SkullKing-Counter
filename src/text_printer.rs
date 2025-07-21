use colored::*;
use std::io::{self, Write};

use crate::{player::{self, Player}, Points};

/// Print a cyan line
fn line() {
    let line = "   O=-===-==-=-------= -=====+=====- =-------=-==-===-=0".cyan();
    println!("{}",line);
}


// Tells to the user to press enter
fn enter() {
    println!("{}","              -= Press [ENTER] to continue =-".blue());    
    io::stdout().flush().unwrap();
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}


// Obtain a i32
fn ask_i32(msg: &str) -> i32 {
    loop {
        let mut guess = String::new();
        print!("{}",msg.bold());
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("{}","Invalid input. Please try again.".red().bold());
            }
        }
    }
}

// Obtain a u8
fn ask_u8(msg: &str) -> u8 {
    loop {
        let mut guess = String::new();
        print!("{}",msg.bold());
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse::<u8>() {
            Ok(num) => return num,
            Err(_) => {
                println!("{}","Invalid input. Please try again.".red().bold());
            }
        }
    }
}

fn ask_yes_no(msg: &str) -> bool {
    loop {
        print!("{} {}:\n", msg.bold(), "(y/n)".yellow());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim().to_lowercase();
            if trimmed == "y" || trimmed == "yes" {
                return true;
            } else if trimmed == "n" || trimmed == "no" {
                return false;
            }
        }
        println!("{}","Please, write 'y'/'yes' or 'n'/'no'.".red().bold());
    }
}

fn ask_string(prompt: &str) -> String {
    use std::io::{self, Write};

    loop {
        print!("{}", prompt.bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if trimmed.is_empty() {
            println!("{}","Text cannot be empty. Please try again.".red().bold());
        } else if trimmed.len() <= 10 && !trimmed.contains('\t') {
            return trimmed.to_string();
        } else if trimmed.contains('\t') {
            println!("{}","Text cannot contain tabs. Please try again.".red().bold());
        } else {
            println!("{}","Text must be 10 characters or fewer. Please try again.".red().bold());
        }
    }
}


// Print a start welcome mensage
pub fn start() {
    let title_skullking = r#"
        ____ _  _ _  _ _    _    _  _ _ _  _ ____ 
        [__  |_/  |  | |    |    |_/  | |\ | | __ 
        ___] | \_ |__| |___ |___ | \_ | | \| |__]
"#.yellow();
    let title_counter = r#"
  _______ _______ ___ ___ ___ __  _______ _______ _______ 
 |   _   |   _   |   Y   |  \ | \|       |   _   |   _   \
 |.  1___|.  |   |.  |   |.  ||  |.|   | |.  1___|.  l   /
 |.  |___|.  |   |.  |   |.      `-|.  |-|.  __)_|.  _   1
 |:  1   |:  1   |:  1   |: |\   | |:  | |:  1   |:  |   |
 |::.. . |::.. . |::.. . |::| |. | |::.| |::.. . |::.|:. |
 `-------`-------`-------`--- ---' `---' `-------`--- ---'
"#.bright_yellow().bold();
    let title_by = r#"
   O=-===-==-=-------= By AxoltDash! =-------=-==-===-=0"#.cyan();

    println!("{}{}{}", title_skullking , title_counter, title_by);
    enter();
}


// Print a ron ron ron msg and need to press enter to continue
pub fn ron_ron_ron() {
    let title_dices = r#"
   ____
  /\' .\   _____
 /: \___\ / .  /\    _   _        _   _        _   _ 
 \' / . //____/..\  |_) / \ |\ | |_) / \ |\ | |_) / \ |\ | 
  \/___/ \'  '\  /  | \ \_/ | \| | \ \_/ | \| | \ \_/ | \|
          \'__'\/

"#.red();

    println!("{}",title_dices);
    line();
    enter();
}

pub fn obtain_players() -> Vec<String> {
    let mut player_list = Vec::new();
    loop {
        let s = ask_string("Please write a player name:\n");
        if !player_list.contains(&s) {
            player_list.push(s);
        } else {
            println!("{}","That name is already in the list. Please enter a different name.".bold().red());
        }
        if !ask_yes_no("Would you like to add another player?") {
            println!();
            break;
        } else { println!(); }
    }
    line();
    player_list
}

pub fn obtain_points(player: String) -> Points {
    let player_colored = player.yellow().bold();

    let spaces = "-".repeat(10 - &player_colored.len());
    let line1 = "   0=-=".cyan();
    let line2 = format!("={}== -=====+=====- ==------=-==-===-=0\n",spaces).cyan();
    
    println!("{} {} {}", line1, player, line2);

    let msg1 = format!("{} - Insert your actually Wins: \n", player_colored);
    let wins = ask_u8(msg1.as_str());
    let msg2 = format!("{} - Insert your tinked Bids: \n", player_colored);
    let binds = ask_u8(msg2.as_str());
    let msg3 = format!("{} - Insert how many extra points you got: \n", player_colored);
    let extra_points = ask_i32(msg3.as_str());

    println!();
    
    Points(wins, binds, extra_points)
}

pub fn stats(list: &Vec<Player>, round: u8) {
    line();
    let p = crate::player::sort_players_desc(&list);
    
    let title1 = format!("--= STATS ROUND {} =--",round).bold();
    

    let pt1 = r#"

              _____________________________
            /  \                           \.
           |    |   "#.yellow();

    let pt2 = r#"   |.
            \_. |                           |.
                | +-==-=--=-= + =-=--=-==-+ |.
                |          |\/.\/|          |.
                |        > ('0 0') <        |.
                | +----=---= ''' =---=----+ |.
                |                           |."#.yellow();

    let head = format!("{}{}{}",pt1,title1,pt2);
    let p3 = "                |   ".yellow();

    println!("{}",head);
    let p4 = "|.".yellow();

    //Printing the players in order
    for (i,p) in p.iter().enumerate() {
        let mut position = (1+&i).to_string();
        let mut player_name = String::from(player::get_name(p));
        if i == 0 {
            position = format!("{}", position.red());
            player_name = format!("{}", player_name.red());
        }
        position.bold();
        let points = player::get_points(p).to_string();
        let spaces = " ".repeat(15 - count_digits(player::get_points(p)) - player::get_name(p).len());
        let stat = format!("{}[{}] - {} : {}{}{}", p3, position, player_name, points, spaces, p4);
        println!("{}", stat);
    }

    let body = r#"                |                           |.
                |   ________________________|___
                |  /                           /.
                \_/___________________________/.

"#.yellow();
    println!("{}",body);

    line();
    enter();
}

fn count_digits(n: i32) -> usize {
    if n < 0 {
        1 + (-n).to_string().len()
    } else {
        n.to_string().len()
    }
}


/// Print the winner
pub fn winner(list: &Vec<Player>) {
    let p = crate::player::sort_players_desc(&list);
    let winner = player::get_name(&p[0]);
    let pt1 = r#"
  /\ .. /\ .. /\    
 |  \/\/  \/\/  |   __      __ __
 |=_-__-__-__-_=|  |  \    /  |__.-----.-----.-----.----.
  |' -_/   _- '|   |.  \/\/   |  |     |     |  -__|   _|
  \  (0)  ( )  /   |.         |__|__|__|__|__|_____|__|   
   '/   /\    '     \:       /   
   \. . .. . ./      \:./\:./"#.yellow();
    let name = format!("   -= {} =-\n", winner);  
    let pt2 = r#"     \/\/\/\/""#.yellow();      

    println!("{}{}{}\n\n",pt1,name,pt2);
    line();
    enter();
}

pub fn play_again() -> bool {
    let msg = "Would you like to play again? (With the same players)";
    if ask_yes_no(msg) {
        println!();
        true
    } else {
        println!("{}","Thank you for playing! See you next time!".green().bold());
        false
    }
}
