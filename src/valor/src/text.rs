use std::rc::Rc;
use std::cell::RefCell;

/// Data structure for instances of text to be rendered.
pub struct Text {
    /// The string to be drawn
    pub data: &'static str,
    /// The vertex data
    pub position: [i32; 2],
    /// The color to draw the text with
    pub color: [f32; 4],
}

/// Utility type for storing Text on a scene
pub type TextHandle = Rc<RefCell<Text>>;

impl Text {
    /// Create a new instance of the text struct
    pub fn new(data: &'static str, position: [i32; 2], color: [f32; 4]) -> TextHandle {
        Rc::new(RefCell::new(Text {
            data,
            position,
            color,
        }))
    }
}
