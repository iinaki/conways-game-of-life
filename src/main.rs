use conways_game_of_life::conways_error::ConwaysError;
use conways_game_of_life::game::Game;

fn main() -> Result<(), ConwaysError>{
    let game = Game::new();
    
    Ok(())
}