/**
 * Tutorial 1 -- Creating a Basic Triangle
 *
 * In this tutorial we learn how to create a window using glutin/glium and use
 * OpenGL to render a triangle.
 **/

// import the crates that we need
#[macro_use]
extern crate glium;
extern crate glutin;

use glium::{DisplayBuild, Surface};


// this struct will hold our vertex data
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

// program entry point
fn main() {
    let display = glutin::WindowBuilder::new()
        .with_title("tetra".to_string())
        .build_glium()
        .unwrap();


    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0, 0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape   = vec![vertex1, vertex2, vertex3];

    let v_buf   = glium::VertexBuffer::new(&display, shape);
    let indices =
        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


    let vertex_shader_src = r#"
        #version 130
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 130
        void main() {
            gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(
        &display, vertex_shader_src, fragment_shader_src, None
    ).unwrap();


    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(
            &v_buf, &indices, &program, &glium::uniforms::EmptyUniforms,
            &std::default::Default::default()
        ).unwrap();
        target.finish();

        display.wait_events().next();
        if display.is_closed() {
            break;
        }
    }
}

