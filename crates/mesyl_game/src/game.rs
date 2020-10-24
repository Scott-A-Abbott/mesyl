
use crate::GameBuilder;
pub struct Game;

impl Game {
    pub fn build() -> GameBuilder {
        GameBuilder{
            game: Game,
        }
    }
}