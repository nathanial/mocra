/**
 * Use React as a model
 * Define a logical tree
 * Which is evaluated and produces a visual tree
 * The visual tree is rendered
 */

use std::collections::BTreeMap;
use std::cell::RefCell;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FontSettings {
    font_size: i32,
    font_weight: FontWeight,
    font_family: FontFamily
}

#[derive(Debug)]
pub enum VisualTreeNode {
    Empty,
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
        font_settings: FontSettings
    }
}

pub trait Widget {
    fn id(&self) -> i32;
    fn bounds(&self) -> Bounds;
    fn render(&self) -> VisualTreeNode;
}

pub type Color = i32;

pub struct Rectangle {
    pub id: i32,
    pub bounds: Bounds,
    pub fill: Color,
    pub stroke: Color,
    pub stroke_width: i32
}

impl Widget for Rectangle {
    fn id(&self) -> i32 {
        self.id
    }
    fn bounds(&self) -> Bounds {
        self.bounds
    }
    fn render(&self) -> VisualTreeNode {
        VisualTreeNode::Rectangle {
            bounds: self.bounds,
            fill: self.fill,
            stroke: self.stroke,
            stroke_width: self.stroke_width,
            children: vec![]
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontWeight {
    Light,
    Normal,
    Bold
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontFamily {
    Arial = 0
}

pub struct Text {
    pub id: i32,
    pub bounds: Bounds,
    pub color: Color,
    pub font_settings: FontSettings,
    pub content: String
}

impl Widget for Text {
    fn id(&self) -> i32 {
        self.id
    }
    fn bounds(&self) -> Bounds {
        self.bounds.clone()
    }
    fn render(&self) -> VisualTreeNode {
        VisualTreeNode::Text {
            content: self.content.clone(),
            bounds: self.bounds,
            color: self.color,
            font_settings: self.font_settings
        }
    }
}

pub struct Button {
    pub id: i32,
    pub bounds: Bounds,
    pub label: String
}

impl Widget for Button {
    fn id(&self) -> i32 {
        self.id
    }
    fn bounds(&self) -> Bounds {
        self.bounds.clone()
    }
    fn render(&self) -> VisualTreeNode {
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
                    font_settings: FontSettings {
                        font_size: 14,
                        font_weight: FontWeight::Normal,
                        font_family: FontFamily::Arial
                    }
                }
            ]
        }
    }
}

type Texture = i32;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct TextRenderSettings {
    bounds: Bounds,
    font_settings: FontSettings,
    content: String
}

thread_local! {
    pub static TEXTURE_CACHE: RefCell<BTreeMap<TextRenderSettings, Texture>> = RefCell::new(BTreeMap::new());
}

fn gl_draw_texture(texture: Texture, bounds: &Bounds){

}

fn create_text_texture(font_settings: &TextRenderSettings) -> Texture {
    panic!("Not Implemented");
}

fn render_text(bounds: &Bounds, font_settings: &FontSettings, content: String) -> Texture {
    let settings = TextRenderSettings {bounds: *bounds, font_settings: *font_settings, content: content};
    TEXTURE_CACHE.with(|cache_cell| {
        let mut cache = cache_cell.borrow_mut();
        match cache.get(&settings) {
            Some(texture) => {
                return *texture;
            },
            None => {}
        }
        let texture = create_text_texture(&settings);
        cache.insert(settings, texture);
        return texture;
    })
}



pub fn render(root: &VisualTreeNode) {
    match root {
        &VisualTreeNode::Empty => {
            println!("Render {:?}", root);
        },
        &VisualTreeNode::Rectangle {bounds, ref children, stroke_width, stroke, fill } => {
            println!("Render {:?}", root);
            for child in children.iter() {
                render(child);
            }
        }
        &VisualTreeNode::Text { ref bounds, color, ref font_settings, ref content } => {
            let texture = render_text(bounds, font_settings, content.to_string());
            gl_draw_texture(texture, bounds);
        }
    }
}
