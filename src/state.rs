use rand::Rng;
use ruscii::terminal::Color;

use crate::{action::ExecutableAction, bot::Bot, resource::Resource};

pub const RESOURCE_RATE: f32 = 0.0002;
pub const RESOURCE_MAX_HEALTH_GAIN: usize = 70;
pub const RESOURCE_MIN_HEALTH_GAIN: usize = 10;
pub const MAP_HEIGHT: usize = 40;
pub const MAP_WIDTH: usize = 80;
pub const BOTS_STARTING_HEALTH: usize = 100;
pub const MOVE_HEALTH_LOSS: usize = 1;

#[derive(Clone, Copy)]
pub enum GameCell {
    Empty,
    Bot(Bot),
    Resource(Resource),
}

pub struct GameState {
    pub map: [[GameCell; MAP_HEIGHT]; MAP_WIDTH],
}

impl GameState {
    pub fn new(bots: Vec<Color>) -> GameState {
        let mut map = [[GameCell::Empty; MAP_HEIGHT]; MAP_WIDTH];

        for bot in bots {
            let mut rng = rand::thread_rng();

            let mut placed = false;

            while !placed {
                let x: usize = rng.gen_range(0..MAP_WIDTH);
                let y: usize = rng.gen_range(0..MAP_HEIGHT);

                if let GameCell::Empty = map[x][y] {
                    map[x][y] = GameCell::Bot(Bot {
                        health: BOTS_STARTING_HEALTH,
                        color: bot,
                    });
                    placed = true;
                }
            }
        }

        GameState { map }
    }

    pub fn update(&mut self) {
        let old_map = self.map.clone();

        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                if let GameCell::Bot(bot) = old_map[x][y] {
                    let action = bot.decide(self);

                    action.execute(x, y, self);
                }
            }
        }
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                if let GameCell::Bot(bot) = self.map[x][y] {
                    if bot.health <= 0 {
                        self.map[x][y] = GameCell::Empty;
                    }
                } else if let GameCell::Empty = self.map[x][y] {
                    let mut rng = rand::thread_rng();
                    if rng.gen_range(0.0..1.0) < RESOURCE_RATE {
                        let health_gain =
                            rng.gen_range(RESOURCE_MIN_HEALTH_GAIN..RESOURCE_MAX_HEALTH_GAIN);

                        self.map[x][y] = GameCell::Resource(Resource { health_gain });
                    }
                }
            }
        }
    }
}
