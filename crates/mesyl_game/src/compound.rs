use crate::GameBuilder;
use std::any::Any;

pub trait Compound: Any + Send + Sync {
    fn build(&self, game: GameBuilder);
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}