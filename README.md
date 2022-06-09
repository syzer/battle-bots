# Battle-Bots: a Rust training

This is a gamified training to introduce Rust in a playful way.

## Welcome!

Welcome to the latest edition of Battle-Bots! As always, we have the greatests bots battling each other:

- Yellow
- Blue
- Green
- Red

Oh-oh... We seem to have technical difficulties... Yellow, Blue and Green are malfunctioning! They appear to only know how to go down!

We need your help!

## Your goals

Go in `src/main.rs`, and follow the instructions from top to bottom:

1. Fix the Yellow bot.
2. Fix the Green bot.
3. Fix the Blue bot.

4. Code your own decision making algorithm for Red, and beat the other colors!


## Rules of the game

- At the beginning of the game, there are 4 bots, battling in a world of 10x20 cells.
- Each bot has an energy level, starting with 9. When the energy level of the Bot reaches 0, the bot loses.
  - The number of the bot in the screen is its energy level.
- Each turn, there is a possibility of resources spawning in any cell. Each resource contains an energy level.  
  - **Resources are white**, and its energy level is its number.
- Each turn, every bot decides which action to take.
- There are 3 actions:
  - Move: bots can only move to one of its **adjacent positions (up, down, left or right)**.
  - Attack: bots can attack bots that are on one adjacent position. 
  - Gather Resource: bots can gather a resource to gain its energy back. Bots can't have more energy than their initial level (9).

## Running a battle

```bash
cargo run
```