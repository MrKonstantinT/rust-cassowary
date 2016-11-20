use math::variables::AbstractVariable;
use math::expressions::Expression;
use objective::problems::Problem;

pub struct Function<T: AbstractVariable> {
    type_of_problem: Problem,
    expression: Expression<T>,
}

impl<T: AbstractVariable> Function<T> {
    pub fn new(p: Problem, e: Expression<T>) -> Function<T> {
        Function {
            expression: e,
            type_of_problem: p,
        }
    }

    pub fn problem(&self) -> &Problem {
        &self.type_of_problem
    }

    pub fn expression(&self) -> &Expression<T> {
        &self.expression
    }
}
