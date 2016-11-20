pub mod functions;

#[cfg(test)]
mod tests {
    use math::variables::{AbstractVariable, Variable};
    use math::relationships::Relationship;
    use math::expressions::Expression;
    use objective::functions::Function;

    #[test]
    fn can_create_functions() {
        let var1: Variable = AbstractVariable::new("Z", 1.0);
        let expression = Expression::new(vec![var1],
                                         Relationship::EQ,
                                         vec![AbstractVariable::new("x", 1.0),
                                              AbstractVariable::new("y", 2.0)]);
        let function = Function::new(expression);
        assert_eq!("Z", function.expression().left_hand_side()[0].name());
        assert_eq!(1.0, function.expression().left_hand_side()[0].coefficient());
        assert_eq!(&Relationship::EQ, function.expression().relationship());
        assert_eq!("x", function.expression().right_hand_side()[0].name());
        assert_eq!(1.0, function.expression().right_hand_side()[0].coefficient());
        assert_eq!("y", function.expression().right_hand_side()[1].name());
        assert_eq!(2.0, function.expression().right_hand_side()[1].coefficient());
    }
}
