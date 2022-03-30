// shaders.rs
// Gets newest shaders for fastest compatible performance
// Tagrets:
//  100
//  140
//  2x0
//  3x0
//  460
//? Should there be so many targets? My crappy laptop with an integrated Intel GPU supports 4.6

// TODO Add shaders
// TODO Allow overwriting target versions
// TODO Better target versions

pub fn get_shader(version: &glium::Version) -> (String, String) {
	let vertex_shader_src: String;
	let fragment_shader_src: String;

	if *version == glium::Version(glium::Api::Gl, 4, 6) {
		// OpenGl 4.6
		vertex_shader_src = r#"
		#version 460
		precision highp float;

		in vec2 position;
		in vec2 tex_coords;

		out vec2 v_tex_coords;

		uniform mat4 matrix;

		void main() {
			v_tex_coords = tex_coords;
			gl_Position = matrix * vec4(position, 0.0, 1.0);
		}
		"#
		.to_string();

		fragment_shader_src = r#"
		#version 460
		precision highp float;

		in vec2 v_tex_coords;

		out vec4 color;

		uniform sampler2D tex;
		
		void main() {
			color = texture(tex, v_tex_coords);
		}
		"#
		.to_string();
	} else if *version > glium::Version(glium::Api::Gl, 1, 4)
		|| *version == glium::Version(glium::Api::Gl, 1, 4)
	{
		// OpenGl 1.4
		vertex_shader_src = r#"
		#version 140
		in vec2 position;
		in vec2 tex_coords;
		out vec2 v_tex_coords;
		uniform mat4 matrix;
		void main() {
			v_tex_coords = tex_coords;
			gl_Position = matrix * vec4(position, 0.0, 1.0);
		}
		"#
		.to_string();

		fragment_shader_src = r#"
		#version 140
		in vec2 v_tex_coords;
		out vec4 color;
		uniform sampler2D tex;
		void main() {
			color = texture(tex, v_tex_coords);
		}
		"#
		.to_string();
	} else {
		// Else fall back to OpenGl 1.0
		vertex_shader_src = r#"
		#version 100
		attribute lowp vec2 position;
		attribute lowp vec2 tex_coords;
		varying lowp vec2 v_tex_coords;
		uniform lowp mat4 matrix;
		void main() {
			v_tex_coords = tex_coords;
			gl_Position = matrix * vec4(position, 0.0, 1.0);
		}
		"#
		.to_string();

		fragment_shader_src = r#"
		#version 100
		varying lowp vec2 v_tex_coords;
		uniform lowp sampler2D tex;
		void main() {
			gl_FragColor = texture2D(tex, v_tex_coords);
		}
		"#
		.to_string();
	}

	(vertex_shader_src, fragment_shader_src)
}
