use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct Tile {
    pub is_walkable: bool,
    pub sprite_id: u32,
}
