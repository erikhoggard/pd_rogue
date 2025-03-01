pub const TILE_SIZE: i32 = 30; // each tile is 20x20 pixels
pub const SCREEN_WIDTH: i32 = 400; // playdate's screen width
pub const SCREEN_HEIGHT: i32 = 240; // playdate's screen height

pub const UI_WIDTH: i32 = 3; // number of columns for the UI area
pub const UI_PIXEL_WIDTH: i32 = UI_WIDTH * TILE_SIZE; // UI area is 40px wide

// Grid dimensions for game area
pub const GRID_WIDTH: i32 = (SCREEN_WIDTH - UI_PIXEL_WIDTH) / TILE_SIZE;
pub const GRID_HEIGHT: i32 = SCREEN_HEIGHT / TILE_SIZE;
