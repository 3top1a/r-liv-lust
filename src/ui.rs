// shaders.rs
// Responsible for all GUI related stuff (so most of the code)
// Beware of spaghetti

extern crate glium;
extern crate image;
extern crate imgui;
extern crate imgui_glium_renderer;

use crate::settings;
use crate::shaders;
use crate::utils;

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
	zoom_level: f32,

	// UI
	debug_menu: bool,
	example_menu: bool,
	metadata_menu: bool,
	action_menu: bool,
}

impl WindowData {
	fn calculate_uniform(&self, window_width: f32, window_height: f32) -> [[f32; 4]; 4]
	{
		let image_width = (self.image_texture.as_ref().unwrap().get_width()) as f32;
		//? Why does height need unwrap() but width doesn't?
		let image_height =
			(self.image_texture.as_ref().unwrap().get_height().unwrap()) as f32;

		let image_ratio = (image_width / image_height) as f32;
		let window_ratio = window_width / window_height as f32;

		// From ArturKovacs/emulsion
		let mut scale_x = 1f32;
		let mut scale_y = 1f32;

		if image_ratio < window_ratio {
			scale_x = ((image_ratio / window_ratio) * window_width)
				.floor() / window_width
		} else {
			scale_y = ((window_ratio / image_ratio) * window_height as f32)
				.floor() / window_height as f32
		}

		[
			[scale_x * self.zoom_level, 0.0, 0.0, 0.0],
			[0.0, scale_y * self.zoom_level, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0f32],
		]
	}

	fn new(filename: String) -> (WindowData, glium::glutin::event_loop::EventLoop<()>) {
		// Default window size
		let width = 800i32;
		let height = 600i32;

		// Set title settings::Settings::WINDOW_TITLE
		// Holy shit is this ever cursed
		let title = format!(
			"{} - {}",
			settings::WindowSettings::WINDOW_TITLE,
			std::path::Path::new(&filename)
				.file_name()
				.unwrap_or_default()
				.to_str()
				.unwrap_or_default()
		);

		// Create OpenGL window
		let event_loop = glium::glutin::event_loop::EventLoop::new();
		let window_builder = glium::glutin::window::WindowBuilder::new()
			.with_title(title)
			.with_decorations(true)
			.with_resizable(true)
			.with_visible(true)
			.with_inner_size(glium::glutin::dpi::LogicalSize::new(width, height));
		let context_builder = glium::glutin::ContextBuilder::new()
			.with_vsync(false) // !Vsync is broken!
			.with_hardware_acceleration(Some(true))
			.with_multisampling(2)
			.with_depth_buffer(0);
		let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

		// Create ImGui
		let mut imgui_builder = imgui::Context::create();

		// Theme
		imgui_builder.set_ini_filename(None);
		imgui_builder.style_mut().use_dark_colors();
		imgui_builder.style_mut().window_rounding = 0.0;
		imgui_builder.style_mut().window_border_size = 1.0;
		imgui_builder.style_mut().alpha = 0.9;
		imgui_builder.style_mut().window_padding = [2.0, 2.0];
		imgui_builder.style_mut().window_title_align = [1.0, 0.5];

		// Make renderer
		let imgui_renderer =
			imgui_glium_renderer::Renderer::init(&mut imgui_builder, &display).unwrap();

		// Get image
		let image = utils::UiUtils::load_texture(&display, filename).unwrap();

		// Auto resize image
		display
			.gl_window()
			.resize(glium::glutin::dpi::PhysicalSize::new(
				image.get_width(),
				image.get_height().unwrap(),
			));

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
				debug_menu: settings::WindowSettings::DEBUG_MENU_OPEN,
				example_menu: false,
				metadata_menu: settings::WindowSettings::METADATA_MENU_OPEN,
				action_menu: true, // No setting for this because it should always be on
				zoom_level: 1.0,
			},
			event_loop,
		)
	}

	fn draw(&mut self) {
		// Create render target
		let mut target = self.gl_display.draw();
		
		// *Draw background
		{
			// Background
			glium::Surface::clear_color(&mut target, 0.05, 0.05, 0.05, 1.0);
		}

		// *Draw image and quad
		{
			// Make quad
			let vertex_buffer =
				{ glium::VertexBuffer::new(&self.gl_display, &utils::UiUtils::QUAD).unwrap() };
			let index_buffer = glium::IndexBuffer::new(
				&self.gl_display,
				glium::index::PrimitiveType::TriangleStrip,
				&[1 as u16, 2, 0, 3],
			)
			.unwrap();

			//Calculate uniform
			let size = self.gl_display.gl_window().window().inner_size();
			self.uniform = self.calculate_uniform(size.width as f32, size.height as f32);

			let uniforms = glium::uniform! {
				matrix: self.uniform,
				tex: self.image_texture.as_ref().unwrap().sampled()
				.wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
				.magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
			};

			// Get shader
			let (vertex_shader, fragment_shader) =
				shaders::get_shader(self.gl_display.get_opengl_version());

			// Create program
			let program = glium::Program::from_source(
				&self.gl_display,
				vertex_shader.as_str(),
				fragment_shader.as_str(),
				None,
			)
			.unwrap();

			// Draw the quad
			glium::Surface::draw(
				&mut target,
				&vertex_buffer,
				&index_buffer,
				&program,
				&uniforms,
				&glium::DrawParameters {
					blend: glium::Blend::alpha_blending(),
					dithering: true,
					backface_culling: glium::BackfaceCullingMode::CullingDisabled,
					..Default::default()
				},
			)
			.unwrap();
		}
	
		// *Draw ImGui
		{
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
						[
							(width as f32 / 2.0) - (350.0 / 2.0),
							height as f32 - 10.0 - 32.0,
						],
						imgui::Condition::FirstUseEver,
					)
					.draw_background(false)
					.scrollable(false)
					.collapsible(false)
					.movable(true)
					.no_decoration()
					.scroll_bar(false)
					.resizable(false)
					.title_bar(false)
					.build(&ui, || {
						ui.separator();
						ui.same_line_with_spacing(0.0, 5.0);
						if ui.button(imgui::im_str!("-"), [32.0, 32.0]) {
							self.zoom_level /= 1.2;
						}
						ui.same_line_with_spacing(0.0, 5.0);
						if ui.button(imgui::im_str!("+"), [32.0, 32.0]) {
							self.zoom_level *= 1.2;
						}
						ui.same_line_with_spacing(0.0, 5.0);
						if ui.button(imgui::im_str!("D"), [32.0, 32.0]) {
							self.debug_menu = !self.debug_menu;
						}
						ui.same_line_with_spacing(0.0, 5.0);
						if ui.button(imgui::im_str!("M"), [32.0, 32.0]) {
							self.metadata_menu = !self.metadata_menu;
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
		
			// Render that ImGui frame to target
			self.im_renderer.render(&mut target, ui.render()).unwrap();
		}

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
					_ => (self.gl_display.gl_window().window().request_redraw()),
				}
			}

			// Resized
			if let glium::glutin::event::Event::WindowEvent { event, .. } = event_ref {
				match event {
					glium::glutin::event::WindowEvent::Resized(..) => {
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

						self.gl_display.gl_window().window().request_redraw()
					}
					glium::glutin::event::WindowEvent::MouseWheel { delta, .. }  => {
						let delta = match delta {
                            glium::glutin::event::MouseScrollDelta::LineDelta(.., y) => { *y },
                            glium::glutin::event::MouseScrollDelta::PixelDelta(d, ..) => {
								(d.x as f32) / 13f32
							}
                        };

						self.zoom_level *= 1.0 + (delta * settings::ImageSettings::ZOOM_MULTIPLIER / 100.0);
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
