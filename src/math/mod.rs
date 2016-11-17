pub mod variables;
pub mod relationships;
pub mod expressions;

#[cfg(test)]
mod tests {
    use math::variables::{AbstractVariable, Variable};
    use math::relationships::Relationship;
    use math::expressions::Expression;

    #[test]
    fn can_create_variables() {
        let v: Variable = Variable::new(String::from("x"), 1.0);
        assert_eq!("x", v.name());
        assert_eq!(1.0, v.coefficient())
    }

    #[test]
    fn can_create_relationships() {
        let equal: Relationship = Relationship::EQ;
        let result = match equal {
            Relationship::EQ => true,
            _ => false,
        };
        assert!(result);
        let less_than_equal: Relationship = Relationship::LEQ;
        let result = match less_than_equal {
            Relationship::LEQ => true,
            _ => false,
        };
        assert!(result);
        let greater_than_equal: Relationship = Relationship::GEQ;
        let result = match greater_than_equal {
            Relationship::GEQ => true,
            _ => false,
        };
        assert!(result);
    }

    #[test]
    fn can_create_expressions() {
        let expression = Expression::new(vec![Variable::new("Z".to_string(), 1.0)],
                                         Relationship::EQ,
                                         vec![Variable::new("x".to_string(), 1.0),
                                              Variable::new("y".to_string(), 2.0)]);
        assert_eq!("Z", expression.left_hand_side()[0].name());
        assert_eq!(1.0, expression.left_hand_side()[0].coefficient());
        assert_eq!(&Relationship::EQ, expression.relationship());
        assert_eq!("x", expression.right_hand_side()[0].name());
        assert_eq!(1.0, expression.right_hand_side()[0].coefficient());
        assert_eq!("y", expression.right_hand_side()[1].name());
        assert_eq!(2.0, expression.right_hand_side()[1].coefficient());
    }
}
