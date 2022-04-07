mod utils;

use matrix_solver::matrix::MatrixState;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(matrix: String) -> String {
    match matrix_solver::parse(&matrix) {
        Ok(m) => {
            let ms = matrix_solver::solve_with_history(m);
            if ms[-1].state() == &MatrixState::Done {
                ms.to_string()
            } else {
                format!("Cannot solve matrix:\n{ms}")
            }
        }
        Err(s) => format!("Failed to parse matrix: {s}"),
    }
}
