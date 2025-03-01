use bevy_ecs::prelude::*;
use pd::graphics::{draw_rect};
use pd::graphics::color::{Color};
use pd::sys::ffi::LCDColor;
use crate::components::{Player, Position, Tile};
use crate::constants::*;
pub fn render_system(
    tile_query: Query<(&Position, &Tile)>,
    player_query: Query<&Position, With<Player>>,
) {
    // Render tile map, offset by UI area (40px)
    // TODO determine this offset dynamically
    for (pos, tile) in tile_query.iter() {
        let color = if tile.is_walkable { Color::WHITE } else { Color::BLACK };
        draw_rect((pos.x * TILE_SIZE) + UI_PIXEL_WIDTH, pos.y * TILE_SIZE, TILE_SIZE, TILE_SIZE, LCDColor::from(color));
    }

    // Render player
    for pos in player_query.iter() {
        draw_rect((pos.x * TILE_SIZE) + UI_PIXEL_WIDTH, pos.y * TILE_SIZE, TILE_SIZE, TILE_SIZE, LCDColor::from(Color::BLACK));
    }
}