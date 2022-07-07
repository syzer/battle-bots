use std::collections::HashMap;

use bot::Bot;
use ruscii::app::{App, Config, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};
use state::{Battle, GameCell, MAP_HEIGHT, MAP_WIDTH};

use self::bot::ColorConfig;
use self::state::{state_to_matrix, STARTING_SHIELD_RESISTANCE};
use self::utils::direction::Direction;

pub mod actuators;
pub mod bot;
pub mod resource;
pub mod state;
pub mod utils;

pub fn run_battle(bots: Vec<ColorConfig>) {
    let mut app = App::config(Config::new().fps(1));

    let mut fps_counter = FPSCounter::new();
    let mut battle = Battle::new(bots);

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        fps_counter.update();
        battle.update();

        let mut pencil = Pencil::new(window.canvas_mut());

        pencil
            .set_origin(Vec2::xy(1 as usize, 1 as usize))
            .set_foreground(Color::Grey)
            .draw_rect(
                &RectCharset::double_lines(),
                Vec2::xy(-1 as isize, -1 as isize),
                Vec2::xy(MAP_WIDTH * 3 + 2, MAP_HEIGHT * 3 + 2),
            );

        let map = state_to_matrix(battle.state.clone());

        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                if let GameCell::Bot(bot) = map[x][y] {
                    pencil.set_foreground(bot.color);
                    pencil.draw_char(
                        format!("{}", bot.energy).as_str().chars().next().unwrap(),
                        Vec2::xy(x * 3, (MAP_HEIGHT - 1 - y) * 3),
                    );
                    let bot_down = Vec2::xy(
                        (x as i32) * 3,
                        ((MAP_HEIGHT as i32) - (y as i32) - 1) * 3 + 1,
                    );
                    let bot_up = Vec2::xy(
                        (x as i32) * 3,
                        ((MAP_HEIGHT as i32) - (y as i32) - 1) * 3 - 1,
                    );
                    let bot_left = Vec2::xy(
                        (x as i32) * 3 - 1,
                        ((MAP_HEIGHT as i32) - (y as i32) - 1) * 3,
                    );
                    let bot_right = Vec2::xy(
                        (x as i32) * 3 + 1,
                        ((MAP_HEIGHT as i32) - (y as i32) - 1) * 3,
                    );

                    if !bot.is_shield_destroyed()
                        && bot.shield_direction.eq(&bot.chainsaw_direction)
                    {
                        match bot.shield_direction {
                            Direction::Down if bot.is_shield_damaged() => {
                                pencil.draw_char('⤈', bot_down)
                            }
                            Direction::Up if bot.is_shield_damaged() => {
                                pencil.draw_char('⤉', bot_up)
                            }
                            Direction::Left if bot.is_shield_damaged() => {
                                pencil.draw_char('⇷', bot_left)
                            }
                            Direction::Right if bot.is_shield_damaged() => {
                                pencil.draw_char('⇸', bot_right)
                            }
                            Direction::Down => pencil.draw_char('⇟', bot_down),
                            Direction::Up => pencil.draw_char('⇞', bot_up),
                            Direction::Left => pencil.draw_char('⇷', bot_left),
                            Direction::Right => pencil.draw_char('⇻', bot_right),
                        };
                    } else {
                        if !bot.is_shield_destroyed() {
                            match bot.shield_direction {
                                Direction::Down if bot.is_shield_damaged() => {
                                    pencil.draw_char('—', bot_down)
                                }
                                Direction::Up if bot.is_shield_damaged() => {
                                    pencil.draw_char('—', bot_up)
                                }
                                Direction::Left if bot.is_shield_damaged() => {
                                    pencil.draw_char('|', bot_left)
                                }
                                Direction::Right if bot.is_shield_damaged() => {
                                    pencil.draw_char('|', bot_right)
                                }

                                Direction::Down => pencil.draw_char('=', bot_down),
                                Direction::Up => pencil.draw_char('=', bot_up),
                                Direction::Left => pencil.draw_char('‖', bot_left),
                                Direction::Right => pencil.draw_char('‖', bot_right),
                            };
                        }
                        match bot.chainsaw_direction {
                            Direction::Down => pencil.draw_char('↓', bot_down),
                            Direction::Up => pencil.draw_char('↑', bot_up),
                            Direction::Left => pencil.draw_char('←', bot_left),
                            Direction::Right => pencil.draw_char('→', bot_right),
                        };
                    }
                } else if let GameCell::Resource(resource) = &map[x][y] {
                    pencil.set_foreground(Color::White);
                    pencil.draw_center_text(
                        format!("{}", resource.energy_gain).as_str(),
                        Vec2::xy(x * 3, (MAP_HEIGHT - 1 - y) * 3),
                    );
                }
            }
        }
    });
}
