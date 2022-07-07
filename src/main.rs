use engine::bot::ColorConfig;
use ruscii::terminal::Color;

mod blue;
mod engine;

fn main() {
    engine::run_battle(vec![
        ColorConfig {
            color: Color::Blue,
            number_of_bots: 3,
            strategy: blue::decide,
        },
        ColorConfig {
            color: Color::Yellow,
            number_of_bots: 3,
            strategy: blue::decide,
        },
        ColorConfig {
            color: Color::Grey,
            number_of_bots: 3,
            strategy: blue::decide,
        },
        ColorConfig {
            color: Color::Red,
            number_of_bots: 3,
            strategy: blue::decide,
        },
    ]);
}
