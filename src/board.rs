use std::collections::HashSet;
use super::cell::Cell;

pub struct Board {
    board: Vec<Vec<Cell>>
}

impl Board {
    pub fn new_from_vec(numbers: Vec<Vec<u32>>) -> Self {
        let mut grid = vec![];
        for row_in in numbers {
            let mut row_out = vec![];
            for item in row_in {
                let cell = Cell::new_from_number(item);
                row_out.push(cell);
            }
            grid.push(row_out);
        }

        Self {
            board: grid
        }
    }

    pub fn to_str_unannotated(&self) -> String {
        let mut output = String::new();

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        output += &(self.board[i*3+j][k*3+l].to_str_unannotated());
                        if l != 2 {
                            output += " ";
                        }
                    }
                    if k != 2 {
                        output += "|";
                    }
                }
                output += "\n";
            }
            if i != 2 {
                output += "-----------------\n"
            }
        }

        output
    } 

    pub fn get_annotations_for_cell(&mut self, i: usize, j: usize) -> HashSet<u32> {
        self.board[i][j].get_annotations()
    }

    fn generate_all_groups(&mut self) -> Vec<Vec<(usize, usize)>> {
        let mut groups = Vec::new();

        for row_idx in 0..9 {
            let mut set = Vec::new();
            for col_idx in 0..9 {
                set.push((row_idx, col_idx));
            }

            groups.push(set);
        }

        for col_idx in 0..9 {
            let mut set = Vec::new();
            for row_idx in 0..9 {
                set.push((row_idx, col_idx));
            }

            groups.push(set);
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut set = Vec::new();

                for k in 0..3 {
                    for l in 0..3 {
                        set.push((i * 3 + k, j * 3 + l));
                    }
                }
                
                groups.push(set);
            }
        }

        groups
    }

    pub fn initial_solve(&mut self) {
        self.annotate_board();

        let mut updated_cell = true;

        while updated_cell {
            println!("Redoing loop!");
            updated_cell = false;

            // Simple solve
            for i in 0..9 {
                for j in 0..9 {
                    let numbers = self.board[i][j].get_annotations();
                    if numbers.len() == 1 {
                        updated_cell = true;
                        let result: u32 = numbers.iter().next().unwrap().clone();
                        self.update_and_propegate_cell(i, j, result);
                        println!("Simple possibility solve - {result} is valid for {}, {}", i, j);
                    }
                }
            }

            // More complex solve
            for group in self.generate_all_groups() {
                updated_cell |= self.resolve_unique_possibilities_for_a_set_of_9(group);
            }

            // This is currently broken
            // Found out this is called "hidden pair"
            // for group in self.generate_all_groups() {
            //     updated_cell |= self.resolve_set_differences_for_set_of_9(group);
            // }
        }        
    }

    fn resolve_unique_possibilities_for_a_set_of_9(&mut self, set_of_coordinates: Vec<(usize, usize)>) -> bool {
        let mut updated_cell = false;

        for num in 1..10 {
            let mut idx = None; 

            for &(i, j) in &set_of_coordinates {

                if self.board[i][j].get_annotations().contains(&num) {
                    if idx == None {
                        idx = Some((i, j));
                    } else {
                        idx = None;
                        break;
                    }
                }
            }

            match idx {
                None => continue,
                Some((i, j)) => {
                    if self.board[i][j].get() == 0 {
                        updated_cell = true;
                        println!("Unique set solve - {num} is valid for {}, {}", i, j);
                        self.update_and_propegate_cell(i, j, num);
                    } else if self.board[i][j].get() != num {
                        panic!("Attempting to overwrite cell, not allowed!");
                    } 
                }
            }
        }

        updated_cell
    }

    fn resolve_set_differences_for_set_of_9(&mut self, set_of_coordinates: Vec<(usize, usize)>) -> bool {
        let mut reversed_sets = Vec::new();
                
        for num in 1..10 {
            let mut set = HashSet::new();
            for &(i, j) in &set_of_coordinates {
                if self.board[i][j].get_annotations().contains(&num) {
                    set.insert((i, j));
                }
            }

            reversed_sets.push(set);
        }

        for num_a in 0..reversed_sets.len() {
            for num_b in 0..reversed_sets.len() {
                let diff = &reversed_sets[num_a] - &reversed_sets[num_b];
                if diff.len() == 1 {
                    let (i, j) = diff.iter().next().unwrap().clone();
                    self.update_and_propegate_cell(i, j, u32::try_from(num_a + 1).unwrap());
                    return true; // Have to return early as all of these need to be recalculated.
                }
            }
        }

        false
    }

    fn update_and_propegate_cell(&mut self, i: usize, j: usize, num: u32) {
        self.board[i][j].set(num);
        self.board[i][j].clear_annotations();

        // Col
        for row_idx in 0..9 {
            if row_idx == i {
                continue;
            }

            self.board[row_idx][j].remove_annotation(num);
        }

        // Row
        for col_idx in 0..9 {
            if col_idx == j {
                continue;
            }

            self.board[i][col_idx].remove_annotation(num);
        }

        // Box
        let base_row_idx = i.div_euclid(3) * 3;
        let base_col_idx = j.div_euclid(3) * 3;

        for row_idx in 0..3 {
            for col_idx in 0..3 {
                let final_row_idx = base_row_idx + row_idx;
                let final_col_idx = base_col_idx + col_idx;

                if final_row_idx == i && final_col_idx == j {
                    continue;
                }
    
                self.board[final_row_idx][final_col_idx].remove_annotation(num);
            }
        }
    }

    pub fn annotate_board(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j].get() == 0 {
                    let mut available_numbers: HashSet<u32> = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
                    available_numbers = &available_numbers - &self.get_used_numbers(i, j);
                    self.board[i][j].set_annotations(available_numbers);
                }
            }
        }
    }

    // Excludes cell itself
    fn get_used_numbers(&self, i: usize, j: usize) -> HashSet<u32> {
        let mut total: HashSet<u32> = HashSet::new();
        total.extend(&self.get_used_numbers_in_col(i, j));
        total.extend(&self.get_used_numbers_in_row(i, j));
        total.extend(&self.get_used_numbers_in_box(i, j));
        total
    }

    // Excludes cell itself
    fn get_used_numbers_in_col(&self, i: usize, j: usize) -> HashSet<u32> {
        let mut numbers = HashSet::new();

        for row_idx in 0..9 {
            if row_idx == i {
                continue;
            }

            numbers.insert(self.board[row_idx][j].get());
        }

        numbers
    }

    // Excludes cell itself
    fn get_used_numbers_in_row(&self, i: usize, j: usize) -> HashSet<u32> {
        let mut numbers = HashSet::new();

        for col_idx in 0..9 {
            if col_idx == j {
                continue;
            }

            numbers.insert(self.board[i][col_idx].get());
        }

        numbers
    }

    // Excludes cell itself
    fn get_used_numbers_in_box(&self, i: usize, j: usize) -> HashSet<u32> {
        let mut numbers = HashSet::new();

        let base_row_idx = i.div_euclid(3) * 3;
        let base_col_idx = j.div_euclid(3) * 3;

        for row_idx in 0..3 {
            for col_idx in 0..3 {
                let final_row_idx = base_row_idx + row_idx;
                let final_col_idx = base_col_idx + col_idx;

                if final_row_idx == i && final_col_idx == j {
                    continue;
                }
    
                numbers.insert(self.board[final_row_idx][final_col_idx].get());
            }
        }

        numbers
    }

}
