use std::cell::RefCell;
use std::fmt::{Formatter, Debug, Result};
use math::variables::new_var;
use math::relationships::Relationship;
use math::expressions::Expression;
use objective::problems::ProblemType;

pub struct Function {
    expression: RefCell<Expression>,
    problem_type: ProblemType,
    expression_max: Option<RefCell<Expression>>,
}

impl Function {
    pub fn new(e: Expression, p_t: ProblemType) -> Function {
        let mut e_m = None;
        if p_t == ProblemType::MIN {
            e_m = Some(RefCell::new(create_expression_to_max(&e)));
        }
        Function {
            expression: RefCell::new(e),
            problem_type: p_t,
            expression_max: e_m,
        }
    }

    pub fn exp(&self) -> &RefCell<Expression> {
        &self.expression
    }

    pub fn p_type(&self) -> &ProblemType {
        &self.problem_type
    }

    pub fn name(&self) -> String {
        if self.problem_type == ProblemType::MAX {
            let exp = self.expression.borrow();
            let last_index = exp.lhs().len() - 1;
            exp.lhs()[last_index].name().clone()
        } else {
            self.expression.borrow().lhs()[0].name().clone()
        }
    }

    pub fn exp_max(&self) -> &RefCell<Expression> {
        if let Some(ref exp_to_max) = self.expression_max {
            &exp_to_max
        } else {
            &self.expression
        }
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let exp = self.expression.borrow();
        let mut lhs_exp_string  = exp.lhs().iter().fold(String::new(), |mut lhs_s, ref var| {
            lhs_s.push_str(" ");
            if var.get_data() != 1.0 {
                lhs_s.push_str(var.get_data().to_string().as_str());
            }
            lhs_s.push_str(var.name());
            lhs_s.push_str(" +");
            lhs_s
        });
        lhs_exp_string.remove(0);
        lhs_exp_string.pop().expect("Failed to pop \"+\" character.");
        lhs_exp_string.pop().expect("Failed to pop \" \" character.");
        let mut rhs_exp_string = exp.rhs().iter().fold(String::new(), |mut rhs_s, ref var| {
            rhs_s.push_str(" ");
            if var.get_data() != 1.0 {
                rhs_s.push_str(var.get_data().to_string().as_str());
            }
            rhs_s.push_str(var.name());
            rhs_s.push_str(" +");
            rhs_s
        });
        rhs_exp_string.pop().expect("Failed to pop \"+\" character.");
        rhs_exp_string.pop().expect("Failed to pop \" \" character.");
        write!(f, "{} ={}", lhs_exp_string, rhs_exp_string)
    }
}

fn create_expression_to_max(expression: &Expression) -> Expression {
    let original_rhs = expression.rhs();
    let mut rhs_max = Vec::with_capacity(original_rhs.len());
    for var in original_rhs {
        let mut var_clone = var.clone();
        var_clone.change_sign();
        rhs_max.push(var_clone);
    }
    Expression::new(vec![new_var("Q", 1.0)], Relationship::EQ, rhs_max)
}
