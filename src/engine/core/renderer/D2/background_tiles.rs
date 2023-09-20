use rand::random;

use glium::Surface;
use glium::implement_vertex;
use glium::texture::{Texture2d, RawImage2d};

#[derive(Copy, Clone)]
struct TileVertex {
    position: [f32; 2],
}

implement_vertex!(TileVertex, position);

pub fn draw_tiles(display: &glium::Display, num_rows: usize, num_cols: usize, square_size: f32) {
    let mut vertex_data = Vec::new();
    let mut index_data = Vec::new();

    for row in 0..num_rows {
        for col in 0..num_cols {
            let color: [f32; 4] = [random(), random(), random(), 1.0];

            let x = col as f32 * square_size;
            let y = row as f32 * square_size;

            vertex_data.push(TileVertex { position: [x, y] });
            index_data.push((row * num_cols + col) as u16);
        }
    }

    let vertex_buffer = glium::VertexBuffer::new(display, &vertex_data).expect("Failed to create vertex buffer");
    let index_buffer = glium::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::Points,
        &index_data,
    ).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        out vec2 tex_coords;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            tex_coords = position;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, tex_coords);
        }
    "#;

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).expect("Failed to create program");

    let image = image::open("assets/texture.png").unwrap().to_rgb8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(display, image).unwrap();

    let uniforms_storage = glium::uniforms::EmptyUniforms;

    let mut target = display.draw();
    target.draw(
        &vertex_buffer,
        &index_buffer,
        &program,
        &uniforms_storage,
        &Default::default(),
    ).unwrap();
    target.finish().unwrap();
}