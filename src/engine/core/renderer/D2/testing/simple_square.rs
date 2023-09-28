use glium::{Frame, implement_vertex, Surface, uniform};


#[derive(Copy, Clone)]
struct MyVertex {
    position: [f32; 2],
}
// you must pass the list of members to the macro
implement_vertex!(MyVertex, position);


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
    let desired_square_size = square_size / window_width as f32;

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
