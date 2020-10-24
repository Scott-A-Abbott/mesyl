pub use {
    mesyl_game as game,
    mesyl_renderer as renderer,
    vek as math,
    bevy_ecs as ecs,
};

pub mod prelude {
    pub use crate::game::prelude::*;
}