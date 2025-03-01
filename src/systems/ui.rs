use pd::graphics::{draw_rect};
use pd::graphics::color::Color;
use pd::sys::ffi::LCDColor;
use crate::constants::*;

pub fn render_ui() {
    draw_rect(0, 0, UI_PIXEL_WIDTH, SCREEN_HEIGHT, LCDColor::from(Color::BLACK)); // UI background
}
