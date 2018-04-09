extern crate gl;
extern crate glutin;

use glutin::GlContext;

/**
 * Use React as a model
 * Define a logical tree
 * Which is evaluated and produces a visual tree
 * The visual tree is rendered
 */

#[derive(Clone, Copy)]
struct Bounds {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

enum VisualTreeNode {
    Rectangle {
        bounds: Bounds,
        fill: Color,
        stroke: Color,
        stroke_width: i32,
        children: Vec<VisualTreeNode>
    },
    Text {
        bounds: Bounds,
        color: Color,
        font_size: i32,
        font_weight: FontWeight,
        font_family: FontFamily
    }
}

trait Widget {
    fn id(&self) -> i32;
    fn bounds(&self) -> Bounds;
    fn render(&mut self) -> VisualTreeNode;
}

type Color = i32;

struct Rectangle {
    id: i32,
    bounds: Bounds,
    fill: Color,
    stroke: Color,
    stroke_width: i32
}

impl Widget for Rectangle {
    fn id(&self) -> i32 {
        self.id
    }
    fn bounds(&self) -> Bounds {
        self.bounds
    }
    fn render(&mut self) -> VisualTreeNode {
        VisualTreeNode::Rectangle {
            bounds: self.bounds,
            fill: self.fill,
            stroke: self.stroke,
            stroke_width: self.stroke_width,
            children: vec![]
        }
    }
}

#[derive(Clone, Copy)]
enum FontWeight {
    Light,
    Normal,
    Bold
}

#[derive(Clone, Copy)]
enum FontFamily {
    Arial
}

struct Text {
    id: i32,
    bounds: Bounds,
    color: Color,
    font_size: i32,
    font_weight: FontWeight,
    font_family: FontFamily
}

impl Widget for Text {
    fn id(&self) -> i32 {
        self.id
    }
    fn bounds(&self) -> Bounds {
        self.bounds.clone()
    }
    fn render(&mut self) -> VisualTreeNode {
        VisualTreeNode::Text {
            bounds: self.bounds,
            color: self.color,
            font_size: self.font_size,
            font_weight: self.font_weight,
            font_family: self.font_family
        }
    }
}

struct Button {
    id: i32,
    bounds: Bounds,
    label: String
}

impl Widget for Button {
    fn id(&self) -> i32 {
        self.id
    }
    fn bounds(&self) -> Bounds {
        self.bounds.clone()
    }
    fn render(&mut self) -> VisualTreeNode {
        VisualTreeNode::Rectangle {
            bounds: self.bounds,
            fill: 0xFFFFFF,
            stroke: 0xFF0000,
            stroke_width: 1,
            children: vec![
                VisualTreeNode::Text {
                    bounds: self.bounds,
                    color: 0x000000,
                    font_size: 14,
                    font_weight: FontWeight::Normal,
                    font_family: FontFamily::Arial
                }
            ]
        }
    }
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                    _ => ()
                },
                _ => ()
            }
        });



        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        gl_window.swap_buffers().unwrap();
    }
}
