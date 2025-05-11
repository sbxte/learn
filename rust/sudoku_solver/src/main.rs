use sudoku_solver::{
    display::Display,
    parse::parse_str_1,
    solver::{SolverOptions, attempt_solve},
};

fn main() {
    let input = include_str!("../input.txt");
    let mut board = parse_str_1(input).unwrap();
    board.display_basic();
    let solved = attempt_solve(&mut board, &SolverOptions::all());
    board.display_full();
    println!("solved: {solved}");
}
