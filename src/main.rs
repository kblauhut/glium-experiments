use glium::glutin;
use glium::Display;
use glium::Surface;
use glium::VertexBuffer;
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use std::fs;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
glium::implement_vertex!(Vertex, position);

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("epic window")
        .with_inner_size(LogicalSize::new(640.0, 480.0));
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &el).unwrap();

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.5],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let fs = fs::read_to_string("src/shaders/fragment/default_fs.glsl").expect("File read error");
    let vs = fs::read_to_string("src/shaders/vertex/default_vs.glsl").expect("File read error");
    let program = glium::Program::from_source(&display, &vs, &fs, None).unwrap();

    el.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(8333333);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        let mut t: f32 = -0.5;
        let mut closed = false;
        while !closed {
            // we update `t`
            t += 0.001;
            if t > 1.0 {
                t = -1.0;
            }
            // drawing
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &glium::uniform! { t: t },
                    &Default::default(),
                )
                .unwrap();
            target.finish().unwrap();
        }
    });
}
