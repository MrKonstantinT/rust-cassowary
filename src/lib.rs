pub mod math;
pub mod objective;
pub mod tableau;

use objective::problems::ProblemType;
use objective::functions::Function;
use objective::constraints::SystemOfConstraints;
use objective::solvers::{transform_constraint_rels_to_eq, rearrange_fun_eq_zero};
use tableau::tables::Table;
use tableau::initials::get_initial_table_from;
use tableau::enter_vars::{enter_var_pivot_optimal, enter_var_pivot_feasible};
use tableau::leave_vars::leave_var;
use tableau::pivots::pivot_around;

pub type Num = f32;

pub fn optimise(function: &mut Function, constraints: &SystemOfConstraints) -> Vec<(String, Num)> {
    rearrange_fun_eq_zero(function);
    match transform_constraint_rels_to_eq(constraints) {
        Ok(_) => {
            // Carry on with Phase II.
            let mut table = get_initial_table_from(function, constraints);
            run_simplex(function, &mut table)
        }
        Err(mut phase1_fun) => {
            rearrange_fun_eq_zero(&mut phase1_fun);
            let mut phase1_table = get_initial_table_from(&phase1_fun, constraints);
            run_simplex(&phase1_fun, &mut phase1_table)
            // Transition
        }
    }
}

fn run_simplex(function: &Function, table: &mut Table) -> Vec<(String, Num)> {
    loop {
        match table.get_basic_solution() {
            Ok(mut basic_solution) => {
                if table.is_solution_optimal() {
                    if function.p_type() == &ProblemType::MIN {
                        // Give solution for MIN as currently it is given as MAX.
                        let (pos, _) = basic_solution.iter()
                            .enumerate()
                            .find(|&entry| (entry.1).0 == "Q")
                            .unwrap();
                        basic_solution[pos] = (function.name().clone(),
                                               basic_solution[pos].1 * -1.0);
                        return basic_solution;
                    } else {
                        return basic_solution;
                    }
                } else {
                    let enter_var_index = enter_var_pivot_optimal(&table);
                    pivot_around(enter_var_index, leave_var(enter_var_index, &table), table);
                }
            }
            Err(index_report) => {
                let enter_var_index =
                    enter_var_pivot_feasible(&table, index_report.0, index_report.1).unwrap();
                pivot_around(enter_var_index, leave_var(enter_var_index, &table), table);
            }
        }
    }
}
