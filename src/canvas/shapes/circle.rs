use crate::canvas::variables::Variable;

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
