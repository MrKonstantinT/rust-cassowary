use ::std::f64;

pub trait AbstractVariable {
    fn new(n: &str, c: f64) -> Self;
    fn name(&self) -> &String;
    fn coefficient(&self) -> f64;
}

pub trait AbstractConstant: AbstractVariable {
    fn value(&self) -> f64;
    fn set_value(&mut self, v: f64);
}

pub struct Variable {
    name: String,
    coefficient: f64,
}

pub struct Constant {
    name: String,
    coefficient: f64,
    value: f64,
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

impl AbstractVariable for Constant {
    fn new(n: &str, c: f64) -> Constant {
        Constant {
            name: n.to_string(),
            coefficient: c,
            value: 0.0,
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn coefficient(&self) -> f64 {
        self.coefficient
    }
}

impl AbstractConstant for Constant {
    fn value(&self) -> f64 {
        self.value
    }

    fn set_value(&mut self, v: f64) {
        self.value = v;
    }
}
