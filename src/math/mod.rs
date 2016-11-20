pub mod variables;
pub mod relationships;
pub mod expressions;

#[cfg(test)]
mod tests {
    use math::variables::{AbstractVariable, AbstractConstant, Variable, Constant};
    use math::relationships::Relationship;
    use math::expressions::Expression;

    #[test]
    fn can_create_variables() {
        let v: Variable = AbstractVariable::new("x", 1.0);
        assert_eq!("x", v.name());
        assert_eq!(1.0, v.coefficient());
    }

    #[test]
    fn can_create_constants() {
        let mut c: Constant = AbstractVariable::new("c", 1.0);
        c.set_value(365.0);
        assert_eq!("c", c.name());
        assert_eq!(1.0, c.coefficient());
        assert_eq!(365.0, c.value());
    }

    #[test]
    fn can_create_relationships() {
        let equal: Relationship = Relationship::EQ;
        assert_eq!(Relationship::EQ, equal);
        let less_than_equal: Relationship = Relationship::LEQ;
        assert_eq!(Relationship::LEQ, less_than_equal);
        let greater_than_equal: Relationship = Relationship::GEQ;
        assert_eq!(Relationship::GEQ, greater_than_equal);
    }

    #[test]
    fn can_create_expressions() {
        let mut c: Constant = AbstractVariable::new("c", 1.0);
        c.set_value(365.0);
        let expression = Expression::new(vec![AbstractVariable::new("Z", 1.0)],
                                         Relationship::EQ,
                                         vec![AbstractVariable::new("x", 1.0),
                                              AbstractVariable::new("y", 2.0),
                                              c]);
        assert_eq!("Z", expression.left_hand_side()[0].name());
        assert_eq!(1.0, expression.left_hand_side()[0].coefficient());
        assert_eq!(Relationship::EQ, *expression.relationship());
        assert_eq!("x", expression.right_hand_side()[0].name());
        assert_eq!(1.0, expression.right_hand_side()[0].coefficient());
        assert_eq!("y", expression.right_hand_side()[1].name());
        assert_eq!(2.0, expression.right_hand_side()[1].coefficient());
        assert_eq!("c", expression.right_hand_side()[2].name());
        assert_eq!(1.0, expression.right_hand_side()[2].coefficient());
        assert_eq!(365.0, expression.right_hand_side()[2].value())
    }
}
