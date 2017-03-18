use math::variables::AbstVar;
use objective::constraints::{Constraint, SystemOfConstraints};
use tableau::tables::Table;
use Num;

pub fn pivot_around(enter_var_index: usize, leave_var_index: usize, table: &mut Table) {
    let row_len = table.get_rows()[leave_var_index].len();
    let pivot_value = table.get_rows()[leave_var_index][enter_var_index];
    // Add multiples of pivot row to other rows to make their pivot column
    // entry 0.0.
    for row_index in 0..table.get_rows().len() {
        if row_index != leave_var_index {
            let value_corres_pivot_column = table.get_rows()[row_index][enter_var_index];
            for i in 0..row_len {
                // value in line of pivot corresponding column of current cell *
                // value in current line corresponding pivot cell column
                let numerator = table.get_rows()[leave_var_index][i] * value_corres_pivot_column;
                table.sub_cell(row_index, i, numerator / pivot_value);
            }
        }
    }
    // Scale pivot row such that the pivot cell becomes 1.0.
    if pivot_value != 1.0 {
        for i in 0..row_len {
            table.div_cell(leave_var_index, i, pivot_value);
        }
    }
}

pub fn apply_transition_rule(a_v_i_s: Vec<(String, Num)>,
                             s_c: &SystemOfConstraints,
                             table: &mut Table) {
    for basic_arti_var in a_v_i_s.iter() {
        let regular_constraints = s_c.system()
            .iter()
            .filter(|constraint| match constraint {
                        &&Constraint::Regular(_) => true,
                        _ => false,
                    })
            .collect::<Vec<&Constraint>>();
        for constraint in regular_constraints {
            match constraint {
                &Constraint::Regular(ref ref_cell) => {
                    let exp = ref_cell.borrow();
                    for var in exp.lhs() {
                        // Pivot on non artificial variable with coefficient not 0.
                        match var {
                            &AbstVar::ArtiVar { .. } => continue,
                            non_arti_var => {
                                if non_arti_var.get_data() != 0.0 {
                                    let row_index = table.get_row_of_basic_var(&basic_arti_var.0);
                                    let column_index = *table.get_column_names()
                                        .get(non_arti_var.name())
                                        .expect("Failed to row number for non-artificial varible.");
                                    pivot_around(column_index, row_index, table);
                                    break;
                                } else {
                                    continue;
                                }
                            }
                        }
                    }
                }
                _ => panic!("Only expected Regular constraints in this iterator."),
            }
        }
    }
}
