// shaders.rs
// Gets newest shaders for fastest compatible performance
// Tagrets:
//  100
//  140
//  2x0
//  3x0
//  4x0
//? Should there be so many targets? My crappy laptop with an integrated Intel GPU supports 4.6

// TODO Add shaders
// TODO Allow overwriting target versions
// TODO Better target versions

pub fn get_shader(version: &glium::Version) -> (String, String)
{
    let vertex_shader_src: String;
    let fragment_shader_src: String;

    if *version > glium::Version(glium::Api::Gl, 1, 4)
    {
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
        "#.to_string();

        fragment_shader_src = r#"
            #version 140
            in vec2 v_tex_coords;
            out vec4 color;
            uniform sampler2D tex;
            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#.to_string();
    }
    else
    {
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
        "#.to_string();

        fragment_shader_src = r#"
            #version 140
            in vec2 v_tex_coords;
            out vec4 color;
            uniform sampler2D tex;
            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#.to_string();
    }

    (
        vertex_shader_src,
        fragment_shader_src,
    )
}
