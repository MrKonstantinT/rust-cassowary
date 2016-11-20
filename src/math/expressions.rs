use math::variables::AbstractVariable;
use math::relationships::Relationship;

pub struct Expression<T: AbstractVariable> {
    left_hand_side: Vec<T>,
    relationship: Relationship,
    right_hand_side: Vec<T>,
}

impl<T: AbstractVariable> Expression<T> {
    pub fn new(l_h_s: Vec<T>, r: Relationship, r_h_s: Vec<T>) -> Expression<T> {
        Expression {
            left_hand_side: l_h_s,
            relationship: r,
            right_hand_side: r_h_s,
        }
    }

    pub fn left_hand_side(&self) -> &Vec<T> {
        &self.left_hand_side
    }

    pub fn relationship(&self) -> &Relationship {
        &self.relationship
    }

    pub fn right_hand_side(&self) -> &Vec<T> {
        &self.right_hand_side
    }

    pub fn multiply_both_sides(&mut self, by: f64) {
        if by.is_sign_negative() && self.relationship != Relationship::EQ {
            if self.relationship == Relationship::GEQ {
                self.relationship = Relationship::LEQ;
            } else {
                self.relationship = Relationship::GEQ;
            }
        }

        multiply_side(&mut self.left_hand_side, by);
        multiply_side(&mut self.right_hand_side, by);
    }
}

fn multiply_side<T: AbstractVariable>(side: &mut Vec<T>, by: f64) {
    for i in 0..side.len() {
        let old_coeff: f64 = side[i].coefficient();
        let variable = &mut side[i];
        variable.set_coefficient(old_coeff * by);
    }
}
