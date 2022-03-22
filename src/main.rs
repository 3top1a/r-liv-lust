mod ui;

fn
main() {
	// Debug
	println!("--- R-liv v? ---");
	println!("ImGui v{}", imgui::dear_imgui_version() );

	// Argument parsing

	let args: Vec<String> = std::env::args().collect();
	if args.len() == 1
	{
		eprintln!("No images selected!");
		std::process::exit(1)
	}
	println!("{:?}", args);

	// Create window and main loop
	ui:: window();

	// Exit
	std::process::exit(0)
}
