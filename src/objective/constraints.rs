use math::variables::AbstractVariable;
use math::expressions::Expression;

pub struct Constraint<T: AbstractVariable> {
    expression: Expression<T>,
}

impl<T: AbstractVariable> Constraint<T> {
    pub fn new(e: Expression<T>) -> Constraint<T> {
        Constraint {
            expression: e,
        }
    }

    pub fn expression(&self) -> &Expression<T> {
        &self.expression
    }
}

pub struct SystemOfConstraints<T: AbstractVariable> {
    system: Vec<Constraint<T>>,
}

impl<T: AbstractVariable> SystemOfConstraints<T> {
    pub fn new() -> SystemOfConstraints<T> {
        SystemOfConstraints {
            system: Vec::new(),
        }
    }

    pub fn get_system(&self) -> &Vec<Constraint<T>> {
        &self.system
    }

    pub fn add_constraint(&mut self, c: Constraint<T>) {
        self.system.push(c);
    }
}
