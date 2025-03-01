use bevy_ecs::prelude::*;
use pd::controls::buttons::PDButtonsExt;
use pd::controls::peripherals::Buttons;
use crate::components::{Position, Player};

pub fn player_movement_system(mut query: Query<&mut Position, With<Player>>) {
    let buttons = Buttons::Default().get();
    if let Ok(mut pos) = query.get_single_mut() {
        if buttons.pushed.left() {
            pos.x -= 1;
        }
        if buttons.pushed.right() {
            pos.x += 1;
        }
        if buttons.pushed.up() {
            pos.y -= 1;
        }
        if buttons.pushed.down() {
            pos.y += 1;
        }
    }
}
