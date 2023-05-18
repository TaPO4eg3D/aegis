use glium::{
    glutin::dpi::PhysicalSize, implement_vertex, Display, Frame, Program,
};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [u32; 2],
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

pub type TakenRegion = Region;

pub struct DrawingContext {
    pub display: Display,
    pub program: Program,
    pub size: PhysicalSize<u32>,
}

pub fn parse_color(color_code: &str) -> [f32; 3] {
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

    fn draw_children(
        children: &Vec<Box<dyn Drawable>>,
        drawing_region: Region,
        context: &DrawingContext,
        frame: &mut Frame
    ) -> TakenRegion {
        let mut available_region = drawing_region;

        for child in children {
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
    pub overflow: bool,
}

pub struct Screen {
    children: Vec<Box<dyn Drawable>>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            children: vec![],
        }
    }
}

impl Container for Screen {
    fn put(&mut self, item: Box<dyn Drawable>) {
        self.children.push(item)
    }
}

impl Drawable for Screen {
    fn set_parent(&mut self, _item: Box<dyn Drawable>) {
        panic!("Cannot set a parent for the Screen!");
    }

    fn draw(&self, drawing_region: Region, context: &DrawingContext, frame: &mut Frame) -> TakenRegion {
        Screen::draw_children(&self.children, drawing_region, context, frame)
   }
}

