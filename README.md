# Sudoku Solver in Rust!
This is an application for me to practice some Rust coding and explore solving methods for Sudoku grids.

# Solving methods implemented:
- Annotate all cells with possible values
- Fill any cell that only has one possible value, propagate this to other cells and resolve 
- In any block, if there is only one cell that contains a given number, fill that cell with that number and propagate

# TODO:
- Guessing - Though would like to avoid if possible