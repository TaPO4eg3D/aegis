use glium::{Frame, Surface, uniform};

use super::base::*;
use super::base::Vertex;


pub struct Rect {
    base_options: BaseOptions,

    children: Vec<Box<dyn Drawable>>,
    parent: Option<Box<dyn Drawable>>,
}

impl Rect {
    pub fn new(base_options: BaseOptions, parent: Option<Box<dyn Drawable>>) -> Self {
        Self {
            base_options,
            children: vec![],
            parent,
        }
    }
}

impl Container for Rect {
    fn put(&mut self, item: Box<dyn Drawable>) {
        self.children.push(item);
    }
}

impl Drawable for Rect {
    fn set_parent(&mut self, item: Box<dyn Drawable>) {
        self.parent = Some(item);
    }

    fn draw(&self, drawing_region: Region, context: &DrawingContext, frame: &mut Frame) -> TakenRegion {
        let opts = &self.base_options;

        let top_left = Vertex { 
            position: [
                drawing_region.p1.x,
                drawing_region.p1.y,
            ] ,
        };
        let top_right = Vertex {
            position: [
                drawing_region.p1.x + opts.width,
                drawing_region.p1.y,
            ],
        };
        let bottom_left = Vertex {
            position: [
                drawing_region.p1.x,
                drawing_region.p1.y + opts.height,
            ],
        };
        let bottom_right = Vertex {
            position: [
                drawing_region.p1.x + opts.width,
                drawing_region.p1.y + opts.height,
            ],
        };

        let taken_space = Region {
            p1: Point{
                x: top_left.position[0],
                y: top_left.position[1],
            },
            p2: Point{
                x: bottom_right.position[0],
                y: bottom_right.position[1],
            },
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

        let color = if let Some(color) = &opts.color {
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

        frame
            .draw(
                &vertex_buffer,
                &indicies,
                &context.program,
                &uniform! {
                    proj: proj,
                    z_index: opts.z_index as i32,
                    usr_color: color,
                },
                &Default::default(),
            )
            .unwrap();


        Rect::draw_children(&self.children, taken_space, context, frame)
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
