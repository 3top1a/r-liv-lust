// shaders.rs
// Responsible for all GUI related stuff (so most of the code)
// Beware of spaghetti

extern crate glium;
extern crate image;
extern crate imgui;
extern crate imgui_glium_renderer;

use crate::shaders;

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 2],
	tex_coords: [f32; 2],
}
glium::implement_vertex!(Vertex, position, tex_coords);

struct WindowData {
	// OpenGl
	//gl_event_loop: glutin::event_loop::EventLoop<()>,
	gl_display: glium::Display,
	uniform: [[f32; 4]; 4],

	// ImGui
	im_builder: imgui::Context,
	im_renderer: imgui_glium_renderer::Renderer,

	// Texture
	image_texture: Option<glium::texture::SrgbTexture2d>,

	// UI
	debug_menu: bool,
	example_menu: bool,
	metadata_menu: bool,
	action_menu: bool,
}

fn load_texture(
	display: &glium::Display,
	filename: String,
) -> Result<glium::texture::SrgbTexture2d, u8> {
	let name = &filename;

	if !std::path::Path::new(name).exists() {
		eprintln!("File doesn't exist!");
		std::process::exit(1);
	}

	// TODO Optimize this loading function
	// `&iimage.into_rgba8()` Why 16 bit? Why alpha?
	// Shouldn't the function be determined seperately?
	// 99% of images are 8bit
	// 80% of images are **not** transparent
	let iimage = image::open(name).unwrap();
	let size = image::image_dimensions(name).unwrap();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
		&iimage.into_rgba8().to_vec(),
		size,
	);
	Ok(
		//? Do we need mipmaps? Won't they just cost performance?
		glium::texture::SrgbTexture2d::with_mipmaps(
			display,
			image,
			glium::texture::MipmapsOption::AutoGeneratedMipmaps,
		)
		.unwrap(),
	)
}

impl WindowData {
	fn new(filename: String) -> (WindowData, glium::glutin::event_loop::EventLoop<()>) {
		// Default window size
		let width = 1024i32;
		let height = 768i32;

		// Create OpenGL window
		let event_loop = glium::glutin::event_loop::EventLoop::new();
		let window_builder = glium::glutin::window::WindowBuilder::new()
			.with_title("Asd")
			.with_decorations(false)
			.with_visible(true)
			.with_inner_size(glium::glutin::dpi::LogicalSize::new(width, height));
		let context_builder = glium::glutin::ContextBuilder::new()
			.with_vsync(false)
			.with_hardware_acceleration(Some(true))
			.with_multisampling(0)
			.with_depth_buffer(0);
		let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

		//display.gl_window().window().set_title(title)
		// Create ImGui
		let mut imgui_builder = imgui::Context::create();

		// Theme
		imgui_builder.set_ini_filename(None);
		imgui_builder.style_mut().use_dark_colors();
		imgui_builder.style_mut().window_rounding = 0.0;
		imgui_builder.style_mut().window_border_size = 1.0;

		// Make renderer
		let imgui_renderer =
			imgui_glium_renderer::Renderer::init(&mut imgui_builder, &display).unwrap();

		// Get image
		let image = load_texture(&display, filename).unwrap();

		// Return data
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
				debug_menu: false,
				example_menu: false,
				metadata_menu: false,
				action_menu: true,
			},
			event_loop,
		)
	}

	fn draw(&mut self) {
		// Create render target
		let mut target = self.gl_display.draw();

		// Background
		glium::Surface::clear_color(&mut target, 0.05, 0.05, 0.05, 1.0);

		// ImGui IO
		let framerate = self.im_builder.io().framerate;
		let delta = self.im_builder.io().delta_time;
		let mut imgui_io = self.im_builder.io_mut();

		// Set display dimentions
		let (width, height) = self.gl_display.get_framebuffer_dimensions();
		imgui_io.display_size = [width as f32, height as f32];

		// Make a frame
		let ui = self.im_builder.frame();

		// Buttons
		if self.action_menu {
			imgui::Window::new(imgui::im_str!("Buttons"))
				.size([350.0, 32.0], imgui::Condition::FirstUseEver)
				.position(
					[(width as f32 / 2.0) - (ui.window_size()[0] / 2.0), height as f32 - 10.0 - 32.0],
					imgui::Condition::FirstUseEver,
				)
				.bg_alpha(0.1)
				.scrollable(false)
				.collapsible(false)
				.movable(true)
				.no_decoration()
				.scroll_bar(false)
				.resizable(false)
				.always_auto_resize(true)
				.title_bar(false)
				.build(&ui, || {
					if ui.button(imgui::im_str!("E"), [32.0, 32.0])
					{
						self.example_menu = !self.example_menu;
					}
					if ui.button(imgui::im_str!("D"), [32.0, 32.0])
					{
						self.debug_menu = !self.debug_menu;
					}
				});
		}

		// Debug window
		if self.debug_menu {
			imgui::Window::new(imgui::im_str!("Debug"))
				.size([350.0, 100.0], imgui::Condition::FirstUseEver)
				.position(
					[(width as f32 / 2f32) - (ui.window_size()[0] / 2.0), 10.0],
					imgui::Condition::Always,
				)
				.bg_alpha(0.1)
				.scrollable(false)
				.collapsible(false)
				.movable(true)
				.no_decoration()
				.scroll_bar(false)
				.resizable(false)
				.title_bar(false)
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
		}

		// Example window
		if self.example_menu {
			imgui::Window::new(imgui::im_str!("Test window"))
				.size([300.0, 100.0], imgui::Condition::FirstUseEver)
				.build(&ui, || {
					ui.text("Hello world!");
					ui.text("This...is...r-liv!");
					ui.separator();
					ui.bullet();
					ui.button(imgui::im_str!("Test"), [60.0, 20.0]);
					ui.separator();
					ui.text("R-liv is made by 3top1a with love!");
					ui.text("It is licensed under AGPL-3.0 License");
				});
		}

		// Make quad
		let vertex_buffer = {
			glium::VertexBuffer::new(
				&self.gl_display,
				&[
					Vertex {
						position: [-1.0, -1.0],
						tex_coords: [0.0, 0.0],
					},
					Vertex {
						position: [-1.0, 1.0],
						tex_coords: [0.0, 1.0],
					},
					Vertex {
						position: [1.0, 1.0],
						tex_coords: [1.0, 1.0],
					},
					Vertex {
						position: [1.0, -1.0],
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

		let uniforms = glium::uniform! {
			matrix: self.uniform,
			tex: self.image_texture.as_ref().unwrap().sampled()
			.wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
			.magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
		};

		// Get shader
		let (vertex_shader, fragment_shader) =
			shaders::get_shader(self.gl_display.get_opengl_version());

		// Print OpenGl version if needed
		//println!("{}", self.gl_display.get_opengl_version_string());

		// Create program
		let program = glium::Program::from_source(
			&self.gl_display,
			vertex_shader.as_str(),
			fragment_shader.as_str(),
			None,
		)
		.unwrap();

		// Draw the quad
		glium::Surface::draw(&mut target, &vertex_buffer, &index_buffer, &program, &uniforms, &glium::DrawParameters {
					blend: glium::Blend::alpha_blending(),
					dithering: true,
					backface_culling: glium::BackfaceCullingMode::CullingDisabled,
					..Default::default()
				})
			.unwrap();

		// Render that ImGui frame to target
		self.im_renderer.render(&mut target, ui.render()).unwrap();

		// End
		target.finish().unwrap();
	}

	fn window_loop(mut self, event: glium::glutin::event_loop::EventLoop<()>) {
		// Loop
		event.run(move |event, _, control_flow| {
			let event_ref = &event;

			// Close
			if let glium::glutin::event::Event::WindowEvent { event, .. } = event_ref {
				match event {
					glium::glutin::event::WindowEvent::CloseRequested
					| glium::glutin::event::WindowEvent::KeyboardInput {
						input:
							glium::glutin::event::KeyboardInput {
								virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::Escape),
								..
							},
						..
					} => {
						*control_flow = glium::glutin::event_loop::ControlFlow::Exit;
						return;
					}
					_ => (),
				}
			}
			std::mem::drop(event_ref);

			// Menus
			if let glium::glutin::event::Event::WindowEvent { event, .. } = event_ref {
				match event {
					// If F2 **pressed**
					glium::glutin::event::WindowEvent::KeyboardInput {
						input:
							glium::glutin::event::KeyboardInput {
								state: glium::glutin::event::ElementState::Pressed,
								virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::F2),
								..
							},
						..
					} => {
						self.debug_menu = !self.debug_menu;
					}
					// If Home **pressed**
					glium::glutin::event::WindowEvent::KeyboardInput {
						input:
							glium::glutin::event::KeyboardInput {
								state: glium::glutin::event::ElementState::Pressed,
								virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::Home),
								..
							},
						..
					} => {
						self.example_menu = !self.example_menu;
					}
					// If space **pressed**
					glium::glutin::event::WindowEvent::KeyboardInput {
						input:
							glium::glutin::event::KeyboardInput {
								state: glium::glutin::event::ElementState::Pressed,
								virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::Space),
								..
							},
						..
					} => {
						self.action_menu = !self.action_menu;
					}
					_ => (),
				}
			}

			// Resized
			if let glium::glutin::event::Event::WindowEvent { event, .. } = event_ref {
				match event {
					glium::glutin::event::WindowEvent::Resized(window_size) => {
						let image_width = (self.image_texture.as_ref().unwrap().get_width()) as f32;
						//? Why does height need unwrap() but width doesn't?
						let image_height =
							(self.image_texture.as_ref().unwrap().get_height().unwrap()) as f32;

						let image_ratio = (image_width / image_height) as f32;
						let window_ratio = window_size.width as f32 / window_size.height as f32;

						// From ArturKovacs/emulsion
						let mut scale_x = 1f32;
						let mut scale_y = 1f32;

						if image_ratio < window_ratio {
							scale_x = ((image_ratio / window_ratio) * window_size.width as f32)
								.floor() / window_size.width as f32
						} else {
							scale_y = ((window_ratio / image_ratio) * window_size.height as f32)
								.floor() / window_size.height as f32
						}

						self.uniform = [
							[scale_x, 0.0, 0.0, 0.0],
							[0.0, scale_y, 0.0, 0.0],
							[0.0, 0.0, 1.0, 0.0],
							[0.0, 0.0, 0.0, 1.0f32],
						];
						self.gl_display.gl_window().window().request_redraw();
					}
					_ => (),
				}
			}

			// Draw
			if let glium::glutin::event::Event::RedrawRequested { .. } = event_ref {
				self.draw();
			}

			// Set mouse stuff
			let mut imgui_io = self.im_builder.io_mut();
			if let glium::glutin::event::Event::WindowEvent { event, .. } = event_ref {
				match event {
					glium::glutin::event::WindowEvent::CursorMoved { position, .. } => {
						// TODO Better mouse movement
						// This has a lot of delay when dragging
						imgui_io.mouse_pos = [position.x as f32, position.y as f32];
					}
					glium::glutin::event::WindowEvent::MouseInput { state, button, .. } => {
						let mut s = false;
						if state == &glium::glutin::event::ElementState::Pressed {
							s = true;
						}

						match button {
							glium::glutin::event::MouseButton::Left => {
								imgui_io.mouse_down = [
									s,
									imgui_io.mouse_down[1],
									imgui_io.mouse_down[2],
									imgui_io.mouse_down[3],
									imgui_io.mouse_down[4],
								];
							}
							glium::glutin::event::MouseButton::Right => {
								imgui_io.mouse_down = [
									imgui_io.mouse_down[0],
									s,
									imgui_io.mouse_down[2],
									imgui_io.mouse_down[3],
									imgui_io.mouse_down[4],
								];
							}
							glium::glutin::event::MouseButton::Middle => {
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
	// Init
	let (data, event_loop) = WindowData::new(filename.to_string());

	// Loop
	data.window_loop(event_loop);
}
