use glium::{Frame, implement_vertex, Surface};


#[derive(Copy, Clone)]
struct MyVertex {
    position: [f32; 2],
}
// you must pass the list of members to the macro
implement_vertex!(MyVertex, position);


fn draw_square_v2(display: glium::Display, frame: &mut Frame) {
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