use glium::{
    glutin::dpi::PhysicalSize, implement_vertex, uniform, Display, Frame, Program, Surface,
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct DrawingContext {
    pub display: Display,
    pub program: Program,
    pub size: PhysicalSize<u32>,
}

fn parse_color(color_code: &str) -> [f32; 3] {
    match color_code {
        "red" => [1., 0., 0.],
        "black" => [0., 0., 0.],
        _ if color_code.starts_with("#") && color_code.len() == 7 => {
            let r = &color_code[1..3];
            let g = &color_code[3..5];
            let b = &color_code[5..7];

            let r = u32::from_str_radix(r, 16).unwrap();
            let g = u32::from_str_radix(g, 16).unwrap();
            let b = u32::from_str_radix(b, 16).unwrap();

            [(r as f32) / 255., (g as f32) / 255., (b as f32) / 255.]
        }
        _ => panic!("Wrong color code!"),
    }
}

pub trait Container {
    fn draw(&self, context: &DrawingContext, frame: &mut Frame) {}
}

pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub color: Option<String>,
    pub z_index: u8,
}

impl Container for Rect {
    fn draw(&self, context: &DrawingContext, frame: &mut Frame) {
        let top_left = Vertex { position: [0., 0.] };
        let top_right = Vertex {
            position: [0. + self.width, 0.],
        };
        let bottom_left = Vertex {
            position: [0., 0. + self.height],
        };
        let bottom_right = Vertex {
            position: [0. + self.width, 0. + self.height],
        };

        let shape = vec![top_right, bottom_right, bottom_left, top_left];
        let vertex_buffer = glium::VertexBuffer::new(&context.display, &shape).unwrap();
        let indicies: [u8; 6] = [0, 1, 3, 1, 2, 3];
        let indicies = glium::index::IndexBuffer::new(
            &context.display,
            glium::index::PrimitiveType::TrianglesList,
            &indicies,
        )
        .unwrap();

        let color = if let Some(color) = &self.color {
            parse_color(color.as_str())
        } else {
            parse_color("black")
        };

        let proj = cgmath::ortho(
            0.,
            context.size.width as f32,
            context.size.height as f32,
            0.,
            0.,
            0.,
        );
        let mut proj: [[f32; 4]; 4] = proj.into();

        proj[2][2] = 1.0;
        proj[2][3] = 1.0;

        println!("{}", context.size.width);
        frame
            .draw(
                &vertex_buffer,
                &indicies,
                &context.program,
                &uniform! {
                    proj: proj,
                    z_index: self.z_index as i32,
                    usr_color: color,
                },
                &Default::default(),
            )
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::parse_color;

    #[test]
    fn test_color_parsing() {
        let red = String::from("red");
        let black = String::from("black");
        let blue_hex = String::from("#4287f5");

        let red_output = parse_color(red.as_str());
        let black_output = parse_color(black.as_str());
        let blue_output = parse_color(blue_hex.as_str());

        assert_eq!(red_output, [1.0, 0.0, 0.0]);
        assert_eq!(black_output, [0.0, 0.0, 0.0]);

        assert_eq!(blue_output, [66.0 / 255., 135.0 / 255., 245.0 / 255.]);
    }
}
