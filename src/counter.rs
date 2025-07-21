use crate::Points;


pub fn calculate_points(p: Points, round: u8) -> i32 {
    if p.0 == 0 && p.1 == 0 {
        bidding_zero(round) + p.2
    } else {
        bidding_one_or_more(&p) + p.2
    }
}

fn bidding_one_or_more(p: &Points) -> i32 {
    if p.0 == p.1 {
        20 * (p.0 as i32)
    } else {
        (p.0.abs_diff(p.1) as i32) * -10
    }
}

fn bidding_zero(round: u8) -> i32 {
    round as i32 * 10
}
