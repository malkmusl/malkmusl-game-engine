use glium::{Frame, implement_vertex, uniform, Display, Surface, VertexBuffer, IndexBuffer};
use glium::texture::SrgbTexture2d;

use crate::engine::console_logger::logger;
use crate::engine::core::entity::player::Player;
use crate::engine::core::metadata;
use crate::engine::core::renderer::core::opengl::OPENGL_DEBUG;

static DEBUG_ONCE: bool = true;
static mut IS_DEBUGED: bool = false; //always false

const VERTEX_SHADER_SRC: &str = r#"
    #version 140
    
    in vec2 position;
    out vec2 v_tex_coords;
            
    uniform mat4 view;
    uniform mat4 camera;
            
    void main() {
        gl_Position = view * camera * vec4(position, 0.0, 1.0);
        v_tex_coords = position;
    }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140
    
    in vec2 v_tex_coords;
    out vec4 color;
    
    uniform sampler2D tex;
    
    void main() {
        color = texture(tex, v_tex_coords);
    }
"#;

#[derive(Copy, Clone)]
pub struct TileVertex {
    position: [f32; 2],
}

#[allow(dead_code)]
pub struct BackgroundTiles {
    display: Display,
    tiles: Vec<Tile>,
    program: glium::Program,
    vertex_buffer: VertexBuffer<TileVertex>,
    index_buffer: IndexBuffer<u16>,
    pub view_matrix: na::Matrix4<f32>,
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

        let view_matrix = set_view_matrix(&display);

        if  is_debugging_enabled(){println!("{}", logger::info_opengl("Creating BackgroundTiles ShaderProgram"))};
        let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None)
            .expect(&logger::error_opengl("Failed to create BackgroundTiles ShaderProgram"));

        if  is_debugging_enabled(){println!("{}", logger::info_opengl("Creating BackgroundTiles VertexBuffer"))};
        let vertex_buffer = glium::VertexBuffer::empty_persistent(&display, 1)
            .expect(&logger::error_opengl("Failed to create BackgroundTiles VertexBuffer"));

        if  is_debugging_enabled(){println!("{}", logger::info_opengl("Creating BackgroundTiles IndexBuffer"))}; 
        let index_buffer = glium::IndexBuffer::empty_persistent(
            &display,
            glium::index::PrimitiveType::TriangleStrip,
            1,
        )
        .expect(&logger::error_opengl("Failed to create BackgroundTiles IndexBuffer"));

        BackgroundTiles {
            display: display.clone(),
            tiles: Vec::new(),
            vertex_buffer,
            index_buffer,
            program,
            view_matrix,
        }
    }

    #[allow(dead_code)]
    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    pub fn draw(
        &mut self,
        frame: &mut Frame,
        rows: usize,
        columns: usize,
        square_size: f32,
        texture: &glium::texture::SrgbTexture2d,
        player: &mut Player,
    ) {
        let mut vertices: Vec<TileVertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

                // Set initial position to (-1, -1)
        let mut x = -1.0;
        let mut y = -1.0;

        // Calculate square size based on the screen dimensions
        let screen_width = 2.0;
        let screen_height = 2.0;

        let square_width = screen_width / columns as f32;
        let square_height = screen_height / rows as f32;

        for i in 0..rows {
            for j in 0..columns {
                // Update x and y for each tile
                x = -1.0 + j as f32 * square_width;
                y = -1.0 + i as f32 * square_height;

                let square_vertices = vec![
                    TileVertex { position: [x, y] },
                    TileVertex { position: [x + square_width, y] },
                    TileVertex { position: [x + square_width, y + square_height] },
                    TileVertex { position: [x, y + square_height] },
                ];

                vertices.extend(square_vertices.iter());

                let base_index = (i * columns + j) * 4;
                let square_indices: Vec<u16> = vec![
                    base_index as u16 + 1,
                    base_index as u16 + 2,
                    base_index as u16,
                    base_index as u16 + 3,
                ];

                indices.extend(square_indices.iter());
            }
        }

        self.vertex_buffer = glium::VertexBuffer::empty_persistent(&self.display, vertices.len())
            .expect(&logger::error_opengl("Failed to create BackgroundTiles VertexBuffer"));
        self.vertex_buffer.write(vertices.as_slice());

        self.index_buffer = glium::IndexBuffer::empty_persistent(
            &self.display,
            glium::index::PrimitiveType::TriangleStrip,
            indices.len(),
        )
        .expect(&logger::error_opengl("Failed to create BackgroundTiles IndexBuffer"));
        self.index_buffer.write(indices.as_slice());

        if is_debugging_enabled(){ 
            println!("{}",format!("{} {}", logger::warn_opengl("Background Vertices length:"), vertices.len()));
            println!("{}",format!("{} {}", logger::warn_opengl("Background Indices length"), indices.len()));

            println!("{}",format!("{} {}", logger::warn_opengl("Background VertexBuffer size:"), self.vertex_buffer.get_size()));
            println!("{}",format!("{} {}", logger::warn_opengl("Background IndexBuffer size:"), self.index_buffer.get_size()));
        }
        self.vertex_buffer.write(vertices.as_slice());

        let camera_matrix = player.update_camera();

        let uniforms = uniform! {
            view: *self.view_matrix.as_ref(),
            camera: *camera_matrix.as_ref(),
            tex: texture,
        };

        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .expect(&logger::error_opengl("Failed to draw BackgroundTiles to Frame"));
        if DEBUG_ONCE {unsafe { IS_DEBUGED = true };}
    }
}

impl Tile {
    #[allow(dead_code)]
    pub fn new(position: [f32; 2], sprite_size: f32, texture: SrgbTexture2d) -> Self {
        Tile {
            position,
            sprite_size,
            texture,
        }
    }
}

pub fn set_view_matrix(display: &Display) -> na::Matrix4<f32> {
    let (window_width, window_height) = display.get_framebuffer_dimensions();
    let aspect_ratio = window_width as f32 / window_height as f32;
    let view_matrix = na::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, aspect_ratio, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
    view_matrix
}

pub fn is_debugging_enabled() -> bool {
    if !DEBUG_ONCE {
        if metadata::DEBUG || unsafe { OPENGL_DEBUG } {
            return true;
        }else{
            return false;
        }
    }else{
        if !unsafe { IS_DEBUGED }{
            if metadata::DEBUG || unsafe { OPENGL_DEBUG } {
                return true;
            }else{
                return false;
            }
        }else{
            return false;
        }
    }
}