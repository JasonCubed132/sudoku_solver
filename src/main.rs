use core::panic;
use std::collections::HashSet;

struct Cell {
    contents: u32,
    annotations: HashSet<u32>
}

impl Cell {
    fn new_blank() -> Self {
        Self { contents: 0, annotations: HashSet::new() }
    }

    fn new_from_number(num: u32) -> Self {
        Self { contents: num, annotations: HashSet::new() }
    }

    fn set(&mut self, num: u32) {
        self.contents = num;
    }

    fn get(&self) -> u32 {
        self.contents
    }

    fn to_str_unannotated(&self) -> String {
        if self.contents == 0 {
            String::from(" ")
        } else {
            self.contents.to_string()
        }
    }

    fn add_annotation(&mut self, num: u32) -> bool {
        self.annotations.insert(num)
    }

    fn remove_annotation(&mut self, num: u32) -> bool {
        self.annotations.remove(&num)
    }

    fn get_annotations(&mut self) -> HashSet<u32> {
        self.annotations.clone()
    }

    fn clear_annotations(&mut self) {
        self.annotations.clear()
    }
}

struct Board {
    board: Vec<Vec<Cell>>
}

impl Board {
    fn new_from_vec(numbers: Vec<Vec<u32>>) -> Self {
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

    fn to_str_unannotated(&self) -> String {
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

    fn initial_solve(&mut self) {
        self.annotate_board();

        let mut updated_cell = true;

        while updated_cell {
            println!("Redoing loop!");
            updated_cell = false;

            // Simple solve
            for i in 0..9 {
                for j in 0..9 {
                    let numbers = self.board[i][j].get_annotations();
                    println!("{:?} {}", numbers, numbers.len());
                    if numbers.len() == 1 {
                        updated_cell = true;
                        let result: u32 = numbers.iter().next().unwrap().clone();
                        self.update_and_propegate_cell(i, j, result);
                    }
                }
            }

            // More complex solve

            // For every row, find the only cell that contains a possible number.
            for row_idx in 0..9 {
                let mut set = Vec::new();
                for col_idx in 0..9 {
                    set.push((row_idx, col_idx));
                }

                updated_cell |= self.resolve_unique_possibilities_for_a_set_of_9(set);
            }

            // For every col, find the only cell that contains a possible number.
            for col_idx in 0..9 {
                let mut set = Vec::new();
                for row_idx in 0..9 {
                    set.push((row_idx, col_idx));
                }

                updated_cell |= self.resolve_unique_possibilities_for_a_set_of_9(set);
            }

            // For every box, find the only cell that contains a possible number
            for i in 0..3 {
                for j in 0..3 {
                    let mut set = Vec::new();

                    for k in 0..3 {
                        for l in 0..3 {
                            set.push((i * 3 + k, j * 3 + l));
                        }
                    }
                    
                    updated_cell |= self.resolve_unique_possibilities_for_a_set_of_9(set);
                }
            }
        }        
    }

    fn resolve_unique_possibilities_for_a_set_of_9(&mut self, set_of_coordinates: Vec<(usize, usize)>) -> bool {
        let mut updated_cell = false;

        for num in 0..9 {
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
                        println!("{num} is valid for {}, {}", i, j);
                        self.update_and_propegate_cell(i, j, num);
                    } else if self.board[i][j].get() != num {
                        panic!("Attempting to overwrite cell, not allowed!");
                    } 
                }
            }
        }

        updated_cell
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

    fn annotate_board(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                let mut available_numbers: HashSet<u32> = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
                available_numbers = &available_numbers - &self.get_used_numbers(i, j);
                self.board[i][j].annotations = available_numbers;
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

fn main() {
    println!("Hello, world!");
    // let mut board = Board::new_from_vec(vec![
    //     vec![0, 3, 0, 0, 7, 0, 0, 0, 4],
    //     vec![6, 4, 9, 0, 3, 1, 0, 0, 7],
    //     vec![8, 0, 0, 0, 0, 0, 6, 0, 0],
    //     vec![0, 0, 0, 1, 0, 0, 0, 0, 0],
    //     vec![0, 0, 0, 0, 0, 6, 0, 4, 5],
    //     vec![7, 5, 3, 0, 0, 0, 0, 9, 0],
    //     vec![0, 6, 2, 0, 0, 0, 3, 0, 8],
    //     vec![0, 7, 0, 9, 0, 0, 0, 6, 1],
    //     vec![0, 0, 5, 7, 0, 0, 0, 0, 9]
    // ]);
    let mut board = Board::new_from_vec(vec![
        vec![2, 9, 7, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 6, 0, 0, 0, 0, 0],
        vec![5, 0, 0, 1, 0, 3, 0, 0, 0],
        vec![0, 7, 0, 0, 0, 0, 0, 5, 0],
        vec![0, 0, 8, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 4, 8, 0, 0, 3, 0],
        vec![0, 0, 5, 0, 0, 2, 0, 0, 7],
        vec![0, 0, 0, 0, 0, 9, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 9, 0, 2]
    ]);

    println!("{}", board.to_str_unannotated());
    board.initial_solve();
    println!("{}", board.to_str_unannotated());
}