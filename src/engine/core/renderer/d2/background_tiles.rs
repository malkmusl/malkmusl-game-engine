use glium::{Frame, implement_vertex, uniform, Display, Surface, Program, VertexBuffer, IndexBuffer};
use glium::texture::SrgbTexture2d;

use crate::engine::core::entity::player::Player;

#[derive(Copy, Clone)]
struct TileVertex {
    position: [f32; 2],
}

#[allow(dead_code)]
pub struct BackgroundTiles {
    display: Display,
    tiles: Vec<Tile>,
    vertex_buffer: VertexBuffer<TileVertex>,
    index_buffer: IndexBuffer<u16>,
}

#[allow(dead_code)]
pub struct Tile {
    pub position: [f32; 2],
    sprite_size: f32,
    texture: SrgbTexture2d,
}

implement_vertex!(TileVertex, position);

impl BackgroundTiles {
    pub fn new(display: Display) -> Self {
        BackgroundTiles {
            display: display.clone(),
            tiles: Vec::new(),
            vertex_buffer: VertexBuffer::empty_dynamic(&display, 4).unwrap(),
            index_buffer: IndexBuffer::empty(&display, glium::index::PrimitiveType::TriangleStrip, 4).unwrap(),
        }
    }

    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    pub fn draw(&mut self, frame: &mut Frame, player: &mut Player) {
        // Clear the frame
        frame.clear_color(0.0, 0.0, 0.0, 0.0);

        for tile in &self.tiles {
            // Update vertex buffer
            let half_size = tile.sprite_size / 2.0;
            let square_vertices = vec![
                TileVertex { position: [tile.position[0] - half_size, tile.position[1] - half_size] },
                TileVertex { position: [tile.position[0] + half_size, tile.position[1] - half_size] },
                TileVertex { position: [tile.position[0] + half_size, tile.position[1] + half_size] },
                TileVertex { position: [tile.position[0] - half_size, tile.position[1] + half_size] },
            ];

            self.vertex_buffer.write(square_vertices.as_slice());

            // Update index buffer
            let square_indices: Vec<u16> = vec![1, 2, 0, 3];
            self.index_buffer.write(square_indices.as_slice());

            // The rest of your code for shaders and drawing remains the same
            let vertex_shader_src = r#"
                #version 140

                in vec2 position;
                out vec2 v_tex_coords; // No explicit texture coordinates

                uniform mat4 view;    // Uniform view matrix for aspect ratio control
                uniform mat4 camera;  // Uniform camera matrix for player movement

                void main() {
                    // Apply both view and camera matrices to the vertex position
                    gl_Position = view * camera * vec4(position, 0.0, 1.0);

                    // Use vertex position as texture coordinates
                    v_tex_coords = position;
                }
            "#;

            let fragment_shader_src = r#"
                #version 140

                in vec2 v_tex_coords; // Receive position as texture coordinates
                out vec4 color;

                uniform sampler2D tex; // Add a texture sampler uniform

                void main() {
                    color = texture(tex, v_tex_coords); // Sample the texture using position as texture coordinates
                }
            "#;

            let program = Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

            // Pass the view matrix and texture as uniforms to the shader
            let uniforms = uniform! {
                view: *player.get_view_matrix().as_ref(),  // Assuming player has a get_view_matrix() method
                camera: *player.update_camera().as_ref(),
                tex: glium::uniforms::Sampler::new(&tile.texture).magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            };

            // Draw the tile
            frame.draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        }
    }
}

impl Tile {
    pub fn new(position: [f32; 2], sprite_size: f32, texture: SrgbTexture2d) -> Self {
        Tile {
            position,
            sprite_size,
            texture,
        }
    }
}
