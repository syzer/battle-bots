use battle_bots_engine::*;

/**
 * Instructions
 * =============
 *
 * At this point, you have fixed all the other bots! Hurray!
 *
 * It's now your turn to implement your own decision making algorithm!
 *
 * Do it by replacing the contents of the `blue` function.
 * - GameState: the size of the map, all the bots and all the resources and their positions
 * - Position: position of the Bot that's deciding what actuators to activate
 *
 * The `blue` function must return an `Actuators` struct, with which you can make the bot:
 * 1. Rotate the chainsaw (Option<Rotation>, if None then there is no rotation)
 * 2. Rotate the shield (Option<Rotation>, if None then there is no rotation)
 * 3. Move the bot (Option<Direction>, if None then the bot doesn't move)
 *
 * Run a battle (`cargo run`) whenever you are ready to test that the blue bot beats the other bots
 */

/**
 * Blue is running a very simple decision making algorithm:
 *
 * - Rotate the shield towards an adjacent enemy
 * - Rotate the chainsaw towards an adjacent enemy, and if it's already in that direction, rotate it Clockwise
 * - If there is no adjacent enemy, move towards the closest one
 */
pub fn blue(game_state: &GameState, bot_position: Position) -> Actuators {
    let Actuators {
        move_bot,
        rotate_chainsaw,
        rotate_shield,
    } = battle_bots_engine::blue(game_state, bot_position);

    Actuators {
        rotate_shield,
        rotate_chainsaw,
        move_bot,
    }
}
