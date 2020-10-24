use {
    mesyl::prelude::*,
    mesyl::math::*,
};

fn main() {
    Game::build();
    let a = Vec3::new(0.0, 1.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);
    println!("Vec A: {}, Vec B: {}, Sum: {}", a, b, a + b);
}