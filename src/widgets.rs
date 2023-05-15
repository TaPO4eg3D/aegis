use glium::{
    glutin::dpi::PhysicalSize, implement_vertex, uniform, Display, Frame, Program, Surface,
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [u32; 2],
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Copy, Clone)]
pub struct Region {
    pub p1: Point,
    pub p2: Point,
}

type TakenRegion = Region;

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
    fn put(&mut self, item: Box<dyn Drawable>);
}

pub trait Drawable {
    fn set_parent(&mut self, item: Box<dyn Drawable>);
    fn draw(&self, drawing_region: Region, context: &DrawingContext, frame: &mut Frame) -> TakenRegion;
}

#[derive(Default)]
pub struct BaseOptions {
    pub width: u32,
    pub height: u32,
    pub z_index: u8,
    pub color: Option<String>,
}

pub struct Screen {
    children: Vec<Box<dyn Drawable>>,
    parent: Option<Box<dyn Drawable>>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            children: vec![],
            parent: None,
        }
    }
}

impl Container for Screen {
    fn put(&mut self, item: Box<dyn Drawable>) {
        self.children.push(item)
    }
}

impl Drawable for Screen {
    fn set_parent(&mut self, item: Box<dyn Drawable>) {
        panic!("Cannot set a parent for the Screen!");
    }

    fn draw(&self, drawing_region: Region, context: &DrawingContext, frame: &mut Frame) -> TakenRegion {
        let mut available_region = drawing_region;

        for child in &self.children {
            let taken_region = child.draw(available_region, context, frame);

            // Screen is only top-down by default
            available_region = Region {
                p1: Point {
                    x: available_region.p1.x,
                    y: available_region.p1.y + taken_region.p2.y,
                },
                p2: Point {
                    x: available_region.p2.x,
                    y: available_region.p2.y,
                }
            }
            
        }

        drawing_region
   }
}

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

        taken_space
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
