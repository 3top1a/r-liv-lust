// settings.rs
// This file is for constants that a user might want to change

pub struct ProgramSettings {}
impl ProgramSettings {
	// Print debug information on start
	// Default: false
	pub const PRINT_DEBUG_INFO: bool = false;
}

pub struct WindowSettings {}
impl WindowSettings {
	// Window title
	// Default: R-Liv
	pub const WINDOW_TITLE: &'static str = "R-Liv";

	// Debug menu open on start up
	// Default: false
	pub const DEBUG_MENU_OPEN: bool = false;

	// Metadata menu open on start up
	// Default: false
	pub const METADATA_MENU_OPEN: bool = false;
}

pub struct ImageSettings {}
impl ImageSettings {
	// Zoom multiplier
	// Default: x
	pub const ZOOM_MULTIPLIER: f32 = 10.0;

}
