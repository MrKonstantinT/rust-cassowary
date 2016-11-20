use math::variables::AbstractVariable;
use math::expressions::Expression;

pub struct Function<T: AbstractVariable> {
    expression: Expression<T>,
}

impl<T: AbstractVariable> Function<T> {
    pub fn new(e: Expression<T>) -> Function<T> {
        Function {
            expression: e,
        }
    }

    pub fn expression(&self) -> &Expression<T> {
        &self.expression
    }
}
