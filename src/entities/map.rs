use bevy_ecs::prelude::*;
use crate::components::{Position, Tile};
use crate::constants::{GRID_WIDTH, GRID_HEIGHT};
use crate::util::random::random_0_to_1;

#[derive(Component)]
pub struct Collision;

#[derive(Component)]
pub enum TileType {
    Floor,
    Wall,
}


pub fn generate_map(world: &mut World) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if random_0_to_1() < 0.3 {
                world.spawn((
                    Position { x, y },
                    Tile {
                        is_walkable: false,
                        sprite_id: 1,
                    },
                    TileType::Wall,
                    Collision,
                ));
            } else {
                world.spawn((
                    Position { x, y },
                    Tile {
                        is_walkable: true,
                        sprite_id: 0,
                    },
                ));
            }
        }
    }
}