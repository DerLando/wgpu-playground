use self::shapes::Shape;

mod shapes;
mod variables;

pub struct Canvas {
    shapes: Vec<Box<dyn Shape>>,
}

impl Canvas {
    pub const fn new() -> Self {
        Self { shapes: Vec::new() }
    }
}
