use std::collections::HashMap;
use std::result::Result;
use Num;

pub struct Table {
    column_names: HashMap<String, usize>, // assume last column reserved
    rows: Vec<Vec<Num>>,
}

impl Table {
    pub fn new(c_n: HashMap<String, usize>, r: Vec<Vec<Num>>) -> Table {
        Table {
            rows: r,
            column_names: c_n,
        }
    }

    pub fn get_column_names(&self) -> &HashMap<String, usize> {
        &self.column_names
    }

    pub fn get_rows(&self) -> &Vec<Vec<Num>> {
        &self.rows
    }

    pub fn get_basic_solution(&self) -> Result<Vec<(String, Num)>, (usize, usize)> {
        let mut basic_solution = Vec::with_capacity(self.column_names.len());
        // Note: ignore RHS column.
        'columns: for i in 0..self.column_names.len() - 1 {
            let mut one_entry_index = 0;
            let mut matched_one = false;
            // Find columns that have exactly one 1.0 and rest 0.0 values...
            'column_entries: for j in 0..self.rows.len() {
                match (matched_one, self.rows[j][i]) {
                    (false, 1.0) | (false, -1.0) => {
                        one_entry_index = j;
                        matched_one = true;
                    }
                    (true, 0.0) | (false, 0.0) => continue 'column_entries,
                    _ => {
                        // ... if this is not the case then the value of this
                        // variables in the basic solution is 0.0.
                        basic_solution.push((get_name_of_index(&self.column_names, i)
                                             .expect("Name not found for index given."),
                                             0.0));
                        continue 'columns;
                    }
                }
            }
            // ... and when we find a basic variable calculate its value but
            // watch out as it might be all zeros so check if we matched a 1.0.
            if matched_one {
                let basic_variable_value = self.rows[one_entry_index][i] *
                                           self.rows[one_entry_index][self.column_names.len() - 1];
                // If the basic variable turns out negative that this solution
                // is not feasable...
                if basic_variable_value.is_sign_negative() &&
                   one_entry_index != self.rows.len() - 1 {
                    // ... report the row where it happened.
                    return Err((one_entry_index, i));
                } else {
                    // ... if not continue generating the solution.
                    basic_solution.push((get_name_of_index(&self.column_names, i)
                                         .expect("get_basic_solution: Name not found for index
                                         given."),
                                         basic_variable_value));
                }
            } else {
                basic_solution.push((get_name_of_index(&self.column_names, i)
                                     .expect("get_basic_solution: Name not found for index
                                     given."), 0.0));
            }
        }
        // If we got here then solution is feasable so return it.
        Ok(basic_solution)
    }

    pub fn get_row_of_basic_var(&self, b_var_name: &String) -> usize {
        let column = *self.column_names.get(b_var_name)
            .expect("Basic variable name supplied does not exist.");
        let mut basic_var_row = 0;
        let mut matched_one = false;
        for i in 0..self.rows.len() {
            match (matched_one, self.rows[i][column]) {
                (false, 0.0) | (true, 0.0) => continue,
                (false, 1.0) => {
                    matched_one = true;
                    basic_var_row = i;
                }
                _ => panic!("Named variable is not basic."),
            }
        }
        basic_var_row
    }

    pub fn is_solution_optimal(&self) -> bool {
        for i in 0..self.column_names.len() - 1 {
            if self.rows[self.rows.len() - 1][i].is_sign_negative() {
                return false;
            }
        }
        true
    }

    pub fn append_row(&mut self, row: Vec<Num>) {
        self.rows.push(row);
    }

    pub fn remove_last_row(&mut self) {
        self.rows.pop().expect("Failed to remove last row from table.");
    }

    pub fn sub_cell(&mut self, row_index: usize, colunm_index: usize, by: Num) {
        self.rows[row_index][colunm_index] = self.rows[row_index][colunm_index] - by;
    }

    pub fn div_cell(&mut self, row_index: usize, colunm_index: usize, by: Num) {
        self.rows[row_index][colunm_index] = self.rows[row_index][colunm_index] / by;
    }
}

fn get_name_of_index(c_n: &HashMap<String, usize>, index: usize) -> Option<String> {
    for (key, val) in c_n.iter() {
        if *val == index {
            return Some(key.clone());
        }
    }
    None
}
