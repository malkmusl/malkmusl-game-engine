use glium::{Frame, implement_vertex, Surface, uniform};

use crate::engine::core::entity::player::Player;


#[derive(Copy, Clone)]
struct MyVertex {
    position: [f32; 2],
}
// you must pass the list of members to the macro
implement_vertex!(MyVertex, position);

#[allow(dead_code)]
pub fn draw_square_v2(display: glium::Display, frame: &mut Frame) {
    let vertex_buffer = glium::VertexBuffer::new(&display, &[
        MyVertex { position: [-0.5, -0.5] },
        MyVertex { position: [0.5, -0.5] },
        MyVertex { position: [0.5, 0.5] },
        MyVertex { position: [-0.5, 0.5] },
    ]).unwrap();

    let index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TriangleStrip,
        &[1 as u16, 2, 0, 3],
    ).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 1.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    frame.draw(
        &vertex_buffer,
        &index_buffer,
        &program,
        &glium::uniforms::EmptyUniforms,
        &Default::default(),
    ).unwrap();
}

#[allow(dead_code)]
pub fn draw_square_grid(        
    display: &glium::Display,
    frame: &mut Frame,
    rows: usize,
    columns: usize,
    square_size: f32,
) {
    let mut vertices: Vec<MyVertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    // Get the current window size
    let (window_width, window_height) = display.get_framebuffer_dimensions();

    // Calculate the aspect ratio
    let aspect_ratio = window_width as f32 / window_height as f32;

    // Calculate the desired square size in window coordinates
    let _desired_square_size = square_size / window_width as f32;

    // Create a view matrix to control the aspect ratio
    let view_matrix = na::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, aspect_ratio, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    // Generate vertices and indices for the grid of squares
    for i in 0..rows {
        for j in 0..columns {
            let x = j as f32 * square_size;
            let y = i as f32 * square_size;

            // Define vertices for the current square
            let square_vertices = vec![
                MyVertex { position: [x, y] },
                MyVertex { position: [x + square_size, y] },
                MyVertex { position: [x + square_size, y + square_size] },
                MyVertex { position: [x, y + square_size] },
            ];

            // Add the square's vertices to the overall vertices vector
            vertices.extend(square_vertices.iter());

            // Calculate the starting index for the current square
            let base_index = (i * columns + j) * 4;

            // Define indices for the current square as a triangle strip
            let square_indices: Vec<u16> = vec![
                base_index as u16 + 1,
                base_index as u16 + 2,
                base_index as u16,
                base_index as u16 + 3,
            ];

            // Add the square's indices to the overall indices vector
            indices.extend(square_indices.iter());
        }
    }

    // Create vertex and index buffers
    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TriangleStrip,
        &indices,
    )
    .unwrap();

    // The rest of your code for shaders and drawing remains the same
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        uniform mat4 view;  // Uniform view matrix for aspect ratio control

        void main() {
            gl_Position = view * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 1.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();
    
    // Pass the view matrix as a uniform to the shader
    let uniforms = uniform! {
        view: *view_matrix.as_ref(),
    };

    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    frame.draw(
        &vertex_buffer,
        &index_buffer,
        &program,
        &uniforms,
        &Default::default(),
    )
    .unwrap();
}
#[allow(dead_code)]
pub fn draw_square_grid_with_texture(
    display: &glium::Display,
    frame: &mut Frame,
    rows: usize,
    columns: usize,
    square_size: f32,
    texture: &glium::texture::SrgbTexture2d, // Add a texture parameter
) {
    let mut vertices: Vec<MyVertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    // Get the current window size
    let (window_width, window_height) = display.get_framebuffer_dimensions();

    // Calculate the aspect ratio
    let aspect_ratio = window_width as f32 / window_height as f32;

    // Calculate the desired square size in window coordinates
    let _desired_square_size = square_size / window_width as f32;

    // Create a view matrix to control the aspect ratio
    let view_matrix = na::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, aspect_ratio, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    // Generate vertices and indices for the grid of squares
    for i in 0..rows {
        for j in 0..columns {
            let x = j as f32 * square_size;
            let y = i as f32 * square_size;

            // Define vertices for the current square without explicit texture coordinates
            let square_vertices = vec![
                MyVertex { position: [x, y] },
                MyVertex { position: [x + square_size, y] },
                MyVertex { position: [x + square_size, y + square_size] },
                MyVertex { position: [x, y + square_size] },
            ];

            // Add the square's vertices to the overall vertices vector
            vertices.extend(square_vertices.iter());

            // Calculate the starting index for the current square
            let base_index = (i * columns + j) * 4;

            // Define indices for the current square as a triangle strip
            let square_indices: Vec<u16> = vec![
                base_index as u16 + 1,
                base_index as u16 + 2,
                base_index as u16,
                base_index as u16 + 3,
            ];

            // Add the square's indices to the overall indices vector
            indices.extend(square_indices.iter());
        }
    }

    // Create vertex and index buffers
    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TriangleStrip,
        &indices,
    )
    .unwrap();

    // The rest of your code for shaders and drawing remains the same
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        out vec2 v_tex_coords; // No explicit texture coordinates

        uniform mat4 view;  // Uniform view matrix for aspect ratio control

        void main() {
            gl_Position = view * vec4(position, 0.0, 1.0);
            v_tex_coords = position; // Use vertex position as texture coordinates
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

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
        .unwrap();
    
    // Pass the view matrix and texture as uniforms to the shader
    let uniforms = uniform! {
        view: *view_matrix.as_ref(),
        tex: texture, // Pass the texture to the shader
    };

    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    frame.draw(
        &vertex_buffer,
        &index_buffer,
        &program,
        &uniforms,
        &Default::default(),
    )
    .unwrap();
}
#[allow(dead_code)]
pub fn draw_square_grid_with_texture_and_player(
    display: &glium::Display,
    frame: &mut Frame,
    rows: usize,
    columns: usize,
    square_size: f32,
    texture: &glium::texture::SrgbTexture2d,
    player: &mut Player // Add a texture parameter
) {
    let mut vertices: Vec<MyVertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    // Get the current window size
    let (window_width, window_height) = display.get_framebuffer_dimensions();

    // Calculate the aspect ratio
    let aspect_ratio = window_width as f32 / window_height as f32;

    // Calculate the desired square size in window coordinates
    let _desired_square_size = square_size / window_width as f32;

    // Create a view matrix to control the aspect ratio
    let view_matrix = na::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, aspect_ratio, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let camera_matrix = player.update_camera();

    // Generate vertices and indices for the grid of squares
    for i in 0..rows {
        for j in 0..columns {
            let x = j as f32 * square_size;
            let y = i as f32 * square_size;

            // Define vertices for the current square without explicit texture coordinates
            let square_vertices = vec![
                MyVertex { position: [x, y] },
                MyVertex { position: [x + square_size, y] },
                MyVertex { position: [x + square_size, y + square_size] },
                MyVertex { position: [x, y + square_size] },
            ];

            // Add the square's vertices to the overall vertices vector
            vertices.extend(square_vertices.iter());

            // Calculate the starting index for the current square
            let base_index = (i * columns + j) * 4;

            // Define indices for the current square as a triangle strip
            let square_indices: Vec<u16> = vec![
                base_index as u16 + 1,
                base_index as u16 + 2,
                base_index as u16,
                base_index as u16 + 3,
            ];

            // Add the square's indices to the overall indices vector
            indices.extend(square_indices.iter());
        }
    }

    // Create vertex and index buffers
    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TriangleStrip,
        &indices,
    )
    .unwrap();

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

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
        .unwrap();
    
    // Pass the view matrix and texture as uniforms to the shader
    let uniforms = uniform! {
        view: *view_matrix.as_ref(),
        camera: *camera_matrix.as_ref(),
        tex: texture, // Pass the texture to the shader
    };

    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    frame.draw(
        &vertex_buffer,
        &index_buffer,
        &program,
        &uniforms,
        &Default::default(),
    )
    .unwrap();
}