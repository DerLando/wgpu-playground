use naga::Handle;

use crate::canvas::variables::Variable;

use super::Shape;

pub struct Circle {
    pub x: Variable,
    pub y: Variable,
    pub radius: Variable,
}

impl Circle {
    pub const fn new() -> Self {
        Self {
            x: Variable::Constant(0.5),
            y: Variable::Constant(0.5),
            radius: Variable::Constant(0.25),
        }
    }
}

impl Shape for Circle {
    fn header_shader() -> &'static str {
        let module = naga::front::wgsl::parse_str(include_str!("circle.wgsl"))
            .expect("Is valid shader file");

        let (header_handle, header_fn) = module
            .functions
            .iter()
            .next()
            .expect("Shader defines a function");
    }
}
