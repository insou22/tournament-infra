use tournament_api::games::round_1::Round1;
use tournament_api::game::Game;

fn main() {
    let game = Round1::new();
    println!("{}", game.get_turn_data().0);
}