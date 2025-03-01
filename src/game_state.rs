use bevy_ecs::prelude::*;
use bevy_ecs::schedule::Schedule;
use crate::entities::{map::generate_map, player::spawn_player};
use crate::systems::{ui::render_ui,player_movement::player_movement_system, render::render_system};

pub struct GameState {
    world: World,
    schedule: Schedule,
}

impl GameState {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut schedule = Schedule::default();

        generate_map(&mut world);
        spawn_player(&mut world);

        schedule.add_systems(player_movement_system);
        schedule.add_systems(render_system);

        Self { world, schedule }
    }

    pub fn update(&mut self) {
        self.schedule.run(&mut self.world);
        render_ui();
    }
}
