use engine::{
    action::Action,
    bot::ColorConfig,
    state::{GameCell, GameState},
    utils::direction::Direction,
};
use ruscii::terminal::Color;

mod blue;
mod engine;

fn main() {
    engine::run_app(vec![
        ColorConfig {
            color: Color::Blue,
            number_of_bots: 5,
            strategy: blue::decide,
        },
        ColorConfig {
            color: Color::Yellow,
            number_of_bots: 5,
            strategy: blue::decide,
        },
        ColorConfig {
            color: Color::Green,
            number_of_bots: 5,
            strategy: blue::decide,
        },
        ColorConfig {
            color: Color::Red,
            number_of_bots: 5,
            strategy: blue::decide,
        },
    ]);
}
