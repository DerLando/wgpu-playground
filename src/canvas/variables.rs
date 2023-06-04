pub enum Variable {
    Constant(f32),
    Variable(BoundedVariable),
}

pub struct BoundedVariable {
    pub name: Option<String>,
    pub min: f32,
    pub max: f32,
}
