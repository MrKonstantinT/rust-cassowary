use ::std::f64;

pub trait AbstractVariable {
    fn new(n: &str, c: f64) -> Self;
    fn name(&self) -> &String;
    fn coefficient(&self) -> f64;
}

pub struct Variable {
    name: String,
    coefficient: f64,
}

impl AbstractVariable for Variable {
    fn new(n: &str, c: f64) -> Variable {
        Variable {
            name: n.to_string(),
            coefficient: c,
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn coefficient(&self) -> f64 {
        self.coefficient
    }
}
