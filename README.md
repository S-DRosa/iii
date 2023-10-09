# iii

## Developers:
- Marcin Pawlukiewicz
- Sergio DeSousa-Rosa Jr.

## Implemented Features:
- Sudoku puzzle validation for rows, columns, and 3x3 subgrids.
- Input sudoku puzzle from an image.
- Ability to check if a sudoku puzzle is valid.

## Help
- Attended Office Hours


## Design Checklist:
- Abstract "Thing" Represented: Two-dimensional arrays, specifically sudoku puzzles, with functions to validate their correctness.
- Functions:
  - `new(width: usize, height: usize) -> Self`: Create a new "Array2" with the given width and height.
  - `from_row_major(data: Vec<(T, (usize, usize))>, width: usize, height: usize) -> Self`: Create an Array2 where the order is organized by rows.
  - `from_column_major(data: Vec<(T, (usize, usize))>, width: usize, height: usize) -> Self`: Create an Array2 where the order is organized by columns.
  - `iter_row_major(&self) -> Option<&T>`: Iterate through the Array2 in row-major order, returning a reference to the value at a given position.
  - `iter_column_major(&self) -> Option<&T>`: Iterate through the Array2 in column-major order, returning a reference to the value at a given position.
  - `get(&self, row: usize, column: usize) -> Option<&T>`: Retrieve a value at a specified position within the Array2.
- Representation and Invariants:
  - Representation: A vector of vectors, where the outer vector represents rows, and the inner vectors represent the values for each column of that row.
  - Invariant: The length must be equal to `width * height`, where `width > 0` and `height > 0`.

## Hours Spent:
We spent approximately 15 hours completing the assignment.
