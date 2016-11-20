pub mod problems;
pub mod functions;
pub mod constraints;
pub mod solver;

#[cfg(test)]
mod tests {
    use math::variables::{AbstractVariable, AbstractConstant, Variable, Constant};
    use math::relationships::Relationship;
    use math::expressions::Expression;
    use objective::problems::Problem;
    use objective::functions::Function;
    use objective::constraints::{Constraint, SystemOfConstraints};

    #[test]
    fn can_create_problems() {
        let max_problem: Problem = Problem::MAX;
        assert_eq!(Problem::MAX, max_problem);
        let min_problem: Problem = Problem::MIN;
        assert_eq!(Problem::MIN, min_problem);
    }

    #[test]
    fn can_create_functions() {
        let var1: Variable = AbstractVariable::new("Z", 1.0);
        let expression = Expression::new(vec![var1],
                                         Relationship::EQ,
                                         vec![AbstractVariable::new("x", 1.0),
                                              AbstractVariable::new("y", 2.0)]);
        let function = Function::new(Problem::MAX, expression);
        assert_eq!(Problem::MAX, *function.problem());
        assert_eq!("Z", function.expression().left_hand_side()[0].name());
        assert_eq!(1.0, function.expression().left_hand_side()[0].coefficient());
        assert_eq!(&Relationship::EQ, function.expression().relationship());
        assert_eq!("x", function.expression().right_hand_side()[0].name());
        assert_eq!(1.0, function.expression().right_hand_side()[0].coefficient());
        assert_eq!("y", function.expression().right_hand_side()[1].name());
        assert_eq!(2.0, function.expression().right_hand_side()[1].coefficient());
    }

    #[test]
    fn can_create_constraints() {
        let mut days_in_year: Constant = AbstractVariable::new("days in year", 1.0);
        days_in_year.set_value(365.0);
        let expression = Expression::new(vec![AbstractVariable::new("x", 1.0),
                                              AbstractVariable::new("y", 2.0)],
                                         Relationship::LEQ,
                                         vec![days_in_year]);
        let constraint = Constraint::new(expression);
        assert_eq!("x", constraint.expression().left_hand_side()[0].name());
        assert_eq!(1.0, constraint.expression().left_hand_side()[0].coefficient());
        assert_eq!("y", constraint.expression().left_hand_side()[1].name());
        assert_eq!(2.0, constraint.expression().left_hand_side()[1].coefficient());
        assert_eq!(&Relationship::LEQ, constraint.expression().relationship());
        assert_eq!("days in year", constraint.expression().right_hand_side()[0].name());
        assert_eq!(1.0, constraint.expression().right_hand_side()[0].coefficient());
        assert_eq!(365.0, constraint.expression().right_hand_side()[0].value());
    }

    #[test]
    fn can_create_system_of_constraints() {
        let mut days_in_year: Constant = AbstractVariable::new("days in year", 1.0);
        days_in_year.set_value(365.0);
        let expression1 = Expression::new(vec![AbstractVariable::new("x", 1.0),
                                              AbstractVariable::new("y", 2.0)],
                                         Relationship::LEQ,
                                         vec![days_in_year]);
        let mut months_in_year: Constant = AbstractVariable::new("months in year", 1.0);
        months_in_year.set_value(12.0);
        let expression2 = Expression::new(vec![AbstractVariable::new("w", 1.0),
                                              AbstractVariable::new("z", 2.0)],
                                         Relationship::LEQ,
                                         vec![months_in_year]);

        let mut system = SystemOfConstraints::new();
        system.add_constraint(Constraint::new(expression1));
        system.add_constraint(Constraint::new(expression2));

        let constraint1 = &system.get_system()[0];
        let constraint2 = &system.get_system()[1];

        assert_eq!("x", constraint1.expression().left_hand_side()[0].name());
        assert_eq!(1.0, constraint1.expression().left_hand_side()[0].coefficient());
        assert_eq!("y", constraint1.expression().left_hand_side()[1].name());
        assert_eq!(2.0, constraint1.expression().left_hand_side()[1].coefficient());
        assert_eq!(&Relationship::LEQ, constraint1.expression().relationship());
        assert_eq!("days in year", constraint1.expression().right_hand_side()[0].name());
        assert_eq!(1.0, constraint1.expression().right_hand_side()[0].coefficient());
        assert_eq!(365.0, constraint1.expression().right_hand_side()[0].value());

        assert_eq!("w", constraint2.expression().left_hand_side()[0].name());
        assert_eq!(1.0, constraint2.expression().left_hand_side()[0].coefficient());
        assert_eq!("z", constraint2.expression().left_hand_side()[1].name());
        assert_eq!(2.0, constraint2.expression().left_hand_side()[1].coefficient());
        assert_eq!(&Relationship::LEQ, constraint2.expression().relationship());
        assert_eq!("months in year", constraint2.expression().right_hand_side()[0].name());
        assert_eq!(1.0, constraint2.expression().right_hand_side()[0].coefficient());
        assert_eq!(12.0, constraint2.expression().right_hand_side()[0].value());
    }
}
