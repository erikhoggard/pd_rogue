use bevy_ecs::prelude::*;
use crate::components::{Position, Player};

pub fn spawn_player(world: &mut World) {
    world.spawn((
        Player,
        Position { x: 5, y: 5 },
    ));
    println!("[INFO] player spawned");
}
