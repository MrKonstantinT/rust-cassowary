use math::variables::AbstractVariable;
use math::relationships::Relationship;
use objective::constraints::SystemOfConstraints;

pub fn convert_greater_constraints<T: AbstractVariable>(system: &mut SystemOfConstraints<T>) {
    for mut constraint in system.get_system().iter() {
        if *constraint.expression().relationship() == Relationship::GEQ {
            //constraint.expression().multiply_both_sides(-1.0);
        }
    }
}
