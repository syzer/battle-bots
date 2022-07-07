use battle_bots_engine::*;

mod blue;

fn main() {
    Battle::new(vec![
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
    ])
    .run()
}
