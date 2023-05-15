use glium::{glutin::dpi::PhysicalSize, implement_vertex, uniform};

use aegis::widgets::{Drawable, Container, DrawingContext, BaseOptions};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        
        uniform int z_index;
        uniform mat4 proj;

        void main() {
            vec4 result = proj * vec4(position, 0.0, 1.0);
            gl_Position = vec4(result[0], result[1], z_index, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        uniform vec3 usr_color;

        void main() {
            color = vec4(usr_color, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut screen = aegis::widgets::Screen::new();

    let rect = aegis::widgets::Rect::new(BaseOptions {
        width: 200,
        height: 200,
        ..Default::default()
    }, None);

    let mut rect2 = aegis::widgets::Rect::new(BaseOptions {
        width: 300,
        height: 300,
        color: Some(String::from("red")),
        ..Default::default()
    }, None);

    let rect3 = aegis::widgets::Rect::new(BaseOptions {
        width: 100,
        height: 100,
        color: Some(String::from("#32a852")),
        ..Default::default()
    }, None);

    rect2.put(Box::new(rect3));

    screen.put(Box::new(rect));
    screen.put(Box::new(rect2));

    let mut drawing_context = DrawingContext {
        display,
        program,
        size: PhysicalSize {
            width: 0,
            height: 0,
        },
    };

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::Resized(window_size) => {
                    drawing_context.size = window_size;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = drawing_context.display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        let drawing_region = aegis::widgets::Region {
            p1: aegis::widgets::Point {
                x: 0,
                y: 0,
            },
            p2: aegis::widgets::Point {
                x: drawing_context.size.width,
                y: drawing_context.size.height,
            },
        };

        screen.draw(drawing_region, &drawing_context, &mut target);
        target.finish().unwrap();
    });
}
