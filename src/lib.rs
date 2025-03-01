#![no_std]

mod components;
mod entities;
mod systems;
mod game_state;
mod constants;
mod util;

extern crate alloc;

#[macro_use]
extern crate playdate as pd;

use core::ffi::*;
use core::ptr::NonNull;
use pd::sys::EventLoopCtrl;
use pd::sys::ffi::PlaydateAPI;
use pd::system::update::UpdateCtrl;
use pd::display::Display;
use pd::graphics::*;
use pd::graphics::color::Color;
use pd::system::prelude::*;
use game_state::GameState;

/// Game state wrapper
struct State {
	game: GameState,
}

impl State {
	fn new() -> Self {
		Self {
			game: GameState::new(),
		}
	}

	/// System event handler
	fn event(&'static mut self, event: SystemEvent) -> EventLoopCtrl {
		match event {
			// Initial setup
			SystemEvent::Init => {
				Display::Default().set_refresh_rate(30.0);
				self.set_update_handler();
				println!("Game init complete");
			},
			_ => {},
		}
		EventLoopCtrl::Continue
	}
}

impl Update for State {
	/// Updates the state
	fn update(&mut self) -> UpdateCtrl {
		clear(Color::WHITE);

		// Update game logic via Bevy ECS
		self.game.update();

		System::Default().draw_fps(0, 0);

		UpdateCtrl::Continue
	}
}

/// Entry point for Playdate
#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
pub fn event_handler(_api: NonNull<PlaydateAPI>, event: SystemEvent, _sim_key_code: u32) -> EventLoopCtrl {
	pub static mut STATE: Option<State> = None;

	if unsafe { STATE.is_none() } {
		let state = State::new();
		unsafe { STATE = Some(state) }
	}

	unsafe { STATE.as_mut().expect("impossible") }.event(event)
}

// Needed for debug build, absolutely optional
ll_symbols!();
