extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;

use glium::*;

/*fn
refresh_mouse()
{
	let mut mouse_pos = (0, 0);
	let mut mouse_pressed = (false, false, false);
	let mut mouse_wheel = 0.0;

	imgui_io.mouse_pos = [mouse_pos.0 as f32 / width as f32, mouse_pos.1 as f32 / height as f32];
	//imgui_io.set_mouse_down(&[mouse_pressed.0, mouse_pressed.1, mouse_pressed.2, false, false]);
	//imgui_io.set_mouse_wheel(mouse_wheel / scale.1);
	mouse_wheel = 0.0;
}*/

pub fn
create_window()
{
	//* Init

	// Create OpenGL window
	let event_loop = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new()
		.with_title("Asd")
		.with_decorations(false)
		.with_inner_size(glutin::dpi::LogicalSize::new(1024f64, 768f64));

	let cb = glutin::ContextBuilder::new()
		.with_vsync(false)
		.with_hardware_acceleration(Some(true))
		.with_multisampling(2);

	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

	// Create ImGui
	let mut imgui_b = imgui::Context::create();
	imgui_b.set_ini_filename(None);
	let mut renderer = imgui_glium_renderer::Renderer::init(&mut imgui_b, &display).unwrap();
	imgui_b.style_mut()
		.use_dark_colors();
	imgui_b.style_mut();

	//* Loop

	event_loop.run(move |event, _, control_flow| {
		// *Events

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

		// *Render

		// Create render target
		let mut target = display.draw();

		// Background
		target.clear_color(0.05, 0.05, 0.05, 1.0);
		
		// ImGui IO
		let imgui_io = imgui_b.io_mut();

		// Set display dimentions
		let (width, height) = display.get_framebuffer_dimensions();
		imgui_io.display_size = [width as f32, height as f32];
		
		// Set mouse stuff
		if let glutin::event::Event::WindowEvent { event , .. } = event_ref
		{
			match event {
				glutin::event::WindowEvent::CursorMoved { device_id, position, modifiers } => {
					// TODO Better mouse movement
					// This has a lot of delay when dragging
					imgui_io.mouse_pos = [position.x as f32, position.y as f32];
				}
				glutin::event::WindowEvent::MouseInput { device_id, state, button, modifiers } =>
				{
					// TODO Better mouse detection
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

		// Make a frame
		let ui = imgui_b.frame();

		// Add a test window
		imgui::Window::new(imgui::im_str!("Hello world"))
			.size([300.0, 100.0], imgui::Condition::FirstUseEver)
			.build(&ui, || {
				ui.text("Hello world!");
				ui.text("This...is...imgui-rs!");
				ui.separator();
			});

		// Render that ImGui frame to target
		renderer.render(&mut target, ui.render()).unwrap();

		target.finish().unwrap();
	});

}