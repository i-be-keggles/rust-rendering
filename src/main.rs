use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

#[macro_use]
extern crate glium;
#[allow(unused_variables)]
fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.finish().unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);
    let shape = vec![
        Vertex { position: [-0.5, -0.5] },
        Vertex { position: [ 0.0,  0.5] },
        Vertex { position: [ 0.5, -0.25] }
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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
            color = vec4(1.0, 0.5, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                &Default::default()).unwrap();
    target.finish().unwrap();

    let _ = event_loop.run(move |event, _, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.set_exit(),
                _ => (),
            },
            _ => (),
        };
    });
}
