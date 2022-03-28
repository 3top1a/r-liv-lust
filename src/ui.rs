extern crate glium;
extern crate image;
extern crate imgui;
extern crate imgui_glium_renderer;


use glium::*;

use crate::shaders;

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 2],
	tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

fn load_texture(display: &glium::Display, filename: String) -> Result<glium::texture::SrgbTexture2d, u8>
{
	let name = &filename;

	if !std::path::Path::new(name).exists()
	{
		return Err(1);
	}

	let iimage = image::open(name).unwrap();
	let size = image::image_dimensions(name).unwrap();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&iimage.as_rgba8().unwrap().to_vec(), size);
	Ok(
		glium::texture::SrgbTexture2d::with_mipmaps(display, image, glium::texture::MipmapsOption::AutoGeneratedMipmapsMax(4)).unwrap()
	)
}

struct WindowData {
	// OpenGl
	//gl_event_loop: glutin::event_loop::EventLoop<()>,
	gl_display: glium::Display,
	uniform: [[f32; 4]; 4],
	
	// ImGui
	im_builder: imgui::Context,
	im_renderer: imgui_glium_renderer::Renderer,
	
	// Etc
	image_texture: Option<glium::texture::SrgbTexture2d>,
}

impl WindowData {
	fn new(filename: String) -> (WindowData, glutin::event_loop::EventLoop<()>) {
		let width = 1024i32;
		let height = 768i32;

		// Create OpenGL window
		let event_loop = glutin::event_loop::EventLoop::new();
		let window_builder = glutin::window::WindowBuilder::new()
			.with_title("Asd")
			.with_decorations(false)
			.with_inner_size(glutin::dpi::LogicalSize::new(width, height));

		let context_builder = glutin::ContextBuilder::new()
			.with_vsync(false)
			.with_hardware_acceleration(Some(true))
			.with_multisampling(0);

		let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

		// Create ImGui
		let mut imgui_builder = imgui::Context::create();

		// Theme
		imgui_builder.set_ini_filename(None);
		imgui_builder.style_mut().use_dark_colors();
		imgui_builder.style_mut().window_rounding = 0.0;
		imgui_builder.style_mut().window_border_size = 1.0;

		let imgui_renderer =
			imgui_glium_renderer::Renderer::init(&mut imgui_builder, &display).unwrap();

		let image = load_texture(&display, filename).unwrap();

		(
			WindowData {
				image_texture: Some(image),
				gl_display: display,
				uniform: [
					[1.0, 0.0, 0.0, 0.0],
					[0.0, 1.0, 0.0, 0.0],
					[0.0, 0.0, 1.0, 0.0],
					[0.0, 0.0, 0.0, 1.0f32],
				],
				im_builder: imgui_builder,
				im_renderer: imgui_renderer,
			},
			event_loop,
		)
	}

	fn draw(&mut self) {
		// Create render target
		let mut target = self.gl_display.draw();

		// Background
		target.clear_color(0.05, 0.05, 0.05, 1.0);

		// ImGui IO
		let framerate = self.im_builder.io().framerate;
		let delta = self.im_builder.io().delta_time;
		let mut imgui_io = self.im_builder.io_mut();

		// Set display dimentions
		let (width, height) = self.gl_display.get_framebuffer_dimensions();
		imgui_io.display_size = [width as f32, height as f32];

		// Make a frame
		let ui = self.im_builder.frame();

		// Add a test window
		imgui::Window::new(imgui::im_str!("Test window"))
			.size([300.0, 100.0], imgui::Condition::FirstUseEver)
			.build(&ui, || {
				ui.text("Hello world!");
				ui.text("This...is...imgui-rs!");
				ui.button(imgui::im_str!("Test"), [60.0, 20.0]);
				ui.separator();
			});

		// Add the debug window
		imgui::Window::new(imgui::im_str!("Debug"))
			.size([350.0, 100.0], imgui::Condition::FirstUseEver)
			.position(
				[(width as f32 / 2f32) - (350.0 / 2.0), 10.0],
				imgui::Condition::Always,
			)
			.bg_alpha(0.25)
			.scrollable(false)
			.collapsible(false)
			.movable(false)
			.no_decoration()
			.scroll_bar(false)
			.resizable(false)
			.build(&ui, || {
				ui.text("Debug menu");
				ui.separator();
				ui.text(format!(
					"Free VRAM: {}MB",
					self.gl_display
						.get_free_video_memory()
						.unwrap_or(usize::MIN) / 1_000_000
				));
				ui.text(format!("Reported FPS: {}", framerate));
				ui.text(format!("Delta: {}", delta));
				ui.text(format!("Calculated FPS: {}", 1.0 / delta));
			});

		// Triangle
		let vertex_buffer = {
			glium::VertexBuffer::new(
				&self.gl_display,
				&[
					Vertex {
						position: [0.0, 0.0],
						tex_coords: [0.0, 0.0],
					},
					Vertex {
						position: [0.0, 1.0],
						tex_coords: [0.0, 1.0],
					},
					Vertex {
						position: [1.0, 1.0],
						tex_coords: [1.0, 1.0],
					},
					Vertex {
						position: [1.0, 0.0],
						tex_coords: [1.0, 0.0],
					},
				],
			)
			.unwrap()
		};
		let index_buffer = glium::IndexBuffer::new(
			&self.gl_display,
			glium::index::PrimitiveType::TriangleStrip,
			&[1 as u16, 2, 0, 3],
		)
		.unwrap();

		let uniforms = uniform! {
			matrix: self.uniform,
			tex: self.image_texture.as_ref().unwrap(),
		};

		let (vertex_shader, fragment_shader) = shaders::get_shader(self.gl_display.get_opengl_version());

		let program = glium::Program::from_source(
			&self.gl_display,
			vertex_shader.as_str(),
			fragment_shader.as_str(),
			None,
		)
		.unwrap();

		target
			.draw(
				&vertex_buffer,
				&index_buffer,
				&program,
				&uniforms,
				&Default::default(),
			)
			.unwrap();

		// Render that ImGui frame to target
		self.im_renderer.render(&mut target, ui.render()).unwrap();

		target.finish().unwrap();
	}

	fn window_loop(mut self, event: glutin::event_loop::EventLoop<()>) {
		event.run(move |event, _, control_flow| {
			let event_ref = &event;

			// Close
			if let glutin::event::Event::WindowEvent { event, .. } = event_ref {
				match event {
					glutin::event::WindowEvent::CloseRequested
					| glutin::event::WindowEvent::KeyboardInput {
						input:
							glutin::event::KeyboardInput {
								virtual_keycode: Some(glutin::event::VirtualKeyCode::Escape),
								..
							},
						..
					} => {
						*control_flow = glutin::event_loop::ControlFlow::Exit;
						return;
					}
					_ => (),
				}
			}
			std::mem::drop(event_ref);

			//* Draw
			if let glutin::event::Event::RedrawRequested { .. } = event_ref {
				self.draw();
			}

			// Set mouse stuff
			let mut imgui_io = self.im_builder.io_mut();
			if let glutin::event::Event::WindowEvent { event, .. } = event_ref {
				match event {
					glutin::event::WindowEvent::CursorMoved { position, .. } => {
						// TODO Better mouse movement
						// This has a lot of delay when dragging
						imgui_io.mouse_pos = [position.x as f32, position.y as f32];
					}
					glutin::event::WindowEvent::MouseInput { state, button, .. } => {
						let mut s = false;
						if state == &glutin::event::ElementState::Pressed {
							s = true;
						}

						match button {
							glutin::event::MouseButton::Left => {
								imgui_io.mouse_down = [
									s,
									imgui_io.mouse_down[1],
									imgui_io.mouse_down[2],
									imgui_io.mouse_down[3],
									imgui_io.mouse_down[4],
								];
							}
							glutin::event::MouseButton::Right => {
								imgui_io.mouse_down = [
									imgui_io.mouse_down[0],
									s,
									imgui_io.mouse_down[2],
									imgui_io.mouse_down[3],
									imgui_io.mouse_down[4],
								];
							}
							glutin::event::MouseButton::Middle => {
								imgui_io.mouse_down = [
									imgui_io.mouse_down[0],
									imgui_io.mouse_down[1],
									s,
									imgui_io.mouse_down[3],
									imgui_io.mouse_down[4],
								];
							}
							_ => (),
						}
					}
					_ => (self.gl_display.gl_window().window().request_redraw()),
				}
			}
			std::mem::drop(imgui_io);
		});
	}
}

pub fn window(filename: &str) {
	//* Init
	let (data, event_loop) = WindowData::new(filename.to_string());

	//* Loop
	data.window_loop(event_loop);
}
