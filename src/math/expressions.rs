use math::variables::AbstractVariable;
use math::relationships::Relationship;

pub struct Expression<T: AbstractVariable> {
    left_hand_side: Vec<T>,
    relationship: Relationship,
    right_hand_side: Vec<T>,
}

impl<T: AbstractVariable> Expression<T> {
    pub fn new(lH: Vec<T>, r: Relationship, rH: Vec<T>) -> Expression<T> {
        Expression {
            left_hand_side: lH,
            relationship: r,
            right_hand_side: rH,
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
}
