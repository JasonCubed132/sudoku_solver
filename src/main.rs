use sudoku_solver::board::Board;

fn main() {
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
    println!("{:?}", board.get_annotations_for_cell(6, 0));
}
