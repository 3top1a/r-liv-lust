mod shaders;
mod ui;
mod utils;

fn main() {
	// Debug
	println!("--- R-liv v{} ---", std::env!("CARGO_PKG_VERSION"));
	println!("ImGui v{}", imgui::dear_imgui_version());

	// Argument parsing

	let args: Vec<String> = std::env::args().collect();
	if args.len() == 1 {
		eprintln!("No images selected!");
		std::process::exit(1)
	}

	// Create window and main loop
	//TODO Better argument parsing
	ui::window(&args[args.len() - 1]);

	// Exit
	std::process::exit(0)
}
