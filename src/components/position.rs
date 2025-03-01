use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
