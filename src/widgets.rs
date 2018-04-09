/**
 * Use React as a model
 * Define a logical tree
 * Which is evaluated and produces a visual tree
 * The visual tree is rendered
 */

#[derive(Clone, Copy, Debug)]
struct Bounds {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

#[derive(Debug)]
enum VisualTreeNode {
    Rectangle {
        bounds: Bounds,
        fill: Color,
        stroke: Color,
        stroke_width: i32,
        children: Vec<VisualTreeNode>
    },
    Text {
        content: String,
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

#[derive(Clone, Copy, Debug)]
enum FontWeight {
    Light,
    Normal,
    Bold
}

#[derive(Clone, Copy, Debug)]
enum FontFamily {
    Arial
}

struct Text {
    id: i32,
    bounds: Bounds,
    color: Color,
    font_size: i32,
    font_weight: FontWeight,
    font_family: FontFamily,
    content: String
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
            content: self.content.clone(),
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
                    content: self.label.clone(),
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


fn render(root: &VisualTreeNode) {
    match root {
        &VisualTreeNode::Rectangle {bounds, ref children, stroke_width, stroke, fill } => {
            println!("Render Rectangle {:?}", root);
            for child in children.iter() {
                render(child);
            }
        }
        &VisualTreeNode::Text { bounds, color, font_size, font_family, font_weight, ref content } => {
            println!("Render Text {:?}", root);
        }
    }
}
