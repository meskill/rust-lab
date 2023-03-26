use n_queens_problem::solve_n_queens;

pub fn main() {
    let solution = solve_n_queens(8, (3, 0)).unwrap_or("None".to_string());

    println!("{solution}")
}
