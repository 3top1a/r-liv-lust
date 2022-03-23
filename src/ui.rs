extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;

use glium::*;

struct WindowData {
	// OpenGl
	
	gl_event_loop: glutin::event_loop::EventLoop<()>,
	gl_display: glium::Display,
	
	// ImGui
	im_builder: imgui::Context,
	im_renderer: imgui_glium_renderer::Renderer,
}

fn
create_window()
-> WindowData
{
	// Create OpenGL window
	let event_loop = glutin::event_loop::EventLoop::new();
	let window_builder = glutin::window::WindowBuilder::new()
		.with_title("Asd")
		.with_decorations(false)
		.with_inner_size(glutin::dpi::LogicalSize::new(1024f64, 768f64));
	
	let context_builder = glutin::ContextBuilder::new()
		.with_vsync(false)
		.with_hardware_acceleration(Some(true))
		.with_multisampling(2);
	
	let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
	
	// Create ImGui
	let mut imgui_builder = imgui::Context::create();
	imgui_builder.set_ini_filename(None);
	imgui_builder.style_mut()
	.use_dark_colors();
	imgui_builder.style_mut();
	
	let imgui_renderer = imgui_glium_renderer::Renderer::init(&mut imgui_builder, &display).unwrap();
	
	WindowData {
		gl_event_loop: event_loop,
		gl_display: display,
		im_builder: imgui_builder,
		im_renderer: imgui_renderer
	}
}

fn
window_loop( mut data: WindowData )
{
	data.gl_event_loop.run(move |event, _, control_flow| {
		let event_ref = &event;
		
		// Close
		if let glutin::event::Event::WindowEvent { event, .. } = event_ref {
			match event {
				glutin::event::WindowEvent::CloseRequested |
				glutin::event::WindowEvent::KeyboardInput {
					input: glutin::event::KeyboardInput {
						virtual_keycode: Some(glutin::event::VirtualKeyCode::Escape),
						..
					},
					..
				} => {
					*control_flow = glutin::event_loop::ControlFlow::Exit;
					return
				},
				_ => (),
			}
		}
		std::mem::drop(event_ref);

		//* Draw
		if let glutin::event::Event::RedrawRequested { .. } = event_ref {
			// Create render target
			let mut target = data.gl_display.draw();
			
			// Background
			target.clear_color(0.05, 0.05, 0.05, 1.0);
			
			// ImGui IO
			let mut imgui_io = data.im_builder.io_mut();
			
			// Set display dimentions
			let (width, height) = data.gl_display.get_framebuffer_dimensions();
			imgui_io.display_size = [width as f32, height as f32];
			
			// Make a frame
			let ui = data.im_builder.frame();

			
			// Add a test window
			imgui::Window::new(imgui::im_str!("Test window"))
			.size([300.0, 100.0], imgui::Condition::FirstUseEver)
			.build(&ui, || {
				ui.text("Hello world!");
				ui.text("This...is...imgui-rs!");
				ui.button(imgui::im_str!("Test"), [60.0, 20.0]);
				ui.separator();
			});
			imgui::Window::new(imgui::im_str!("Debug"))
			.size([350.0, 60.0], imgui::Condition::FirstUseEver)
			.position([(width / 2) as f32, 10.0], imgui::Condition::Always)
			.bg_alpha(0.25)
			.scrollable(false)
			.collapsible(false)
			.movable(false)
			.no_decoration()
			.scroll_bar(false)
			.resizable(false)
			.build(&ui, || {
				ui.text("--- Debug menu ---");
				ui.separator();
				ui.text( format!("Free VRAM: {}KB", data.gl_display.get_free_video_memory().unwrap_or(usize::MIN)));
				//ui.text( format!("Reported FPS: {}", &imgui_io.framerate  ))
			});
			
			// Render that ImGui frame to target
			data.im_renderer.render(&mut target, ui.render()).unwrap();
			
			target.finish().unwrap();
		}
		
		// Set mouse stuff
		let mut imgui_io = data.im_builder.io_mut();
		if let glutin::event::Event::WindowEvent { event , .. } = event_ref
		{
			match event {
				glutin::event::WindowEvent::CursorMoved { position , ..} => {
					// TODO Better mouse movement
					// This has a lot of delay when dragging
					imgui_io.mouse_pos = [position.x as f32, position.y as f32];
				}
				glutin::event::WindowEvent::MouseInput { state, button, ..} =>
				{
					// TODO Better mouse input
					// This is really jank and is only for M1
					if button == &glutin::event::MouseButton::Left
					{
						let mut s = false;
						if state == &glutin::event::ElementState::Pressed
						{
							s = true;
						}
						
						imgui_io.mouse_down = [ s , false, false, false, false ]
					}
				}
				_ => ()
			}
		}
		std::mem::drop(imgui_io);
	});
}

pub fn
window()
{
	//* Init
	let data = create_window();
	
	//* Loop
	window_loop(data);
}
