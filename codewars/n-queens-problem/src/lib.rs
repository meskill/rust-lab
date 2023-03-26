use rand::{thread_rng, Rng};

fn shuffle<T>(arr: &mut [T]) {
    let n = arr.len();
    let mut rng = thread_rng();
    for i in 0..n - 1 {
        let pos = rng.gen_range(i..n);

        arr.swap(i, pos);
    }
}

fn num_collisions(diags: &[usize], diag: usize, diag_2: usize) -> usize {
    diags[diag] - 1 + if diag == diag_2 { 0 } else { diags[diag_2] - 1 }
}

fn num_collisions_new(diags: &[usize], diag: usize, diag_2: usize) -> usize {
    diags[diag] + if diag == diag_2 { 1 } else { diags[diag_2] }
}

/**
 * Solution is based on [paper](https://citeseerx.ist.psu.edu/viewdoc/download;jsessionid=4DC9292839FE7B1AFABA1EDB8183242C?doi=10.1.1.57.4685&rep=rep1&type=pdf)
 */
pub fn solve_n_queens(n: usize, mandatory_coords: (usize, usize)) -> Option<String> {
    let (mandatory_col, mandatory_row) = mandatory_coords;

    // pick a random number of iterations to validate different permutations
    for _ in 0..(if n > 20 { 10 } else { 100 }) {
        let mut permutation: Vec<_> = (0..n).collect();
        let mut main_diags = vec![0usize; 2 * n - 1];
        let mut second_diags = vec![0usize; 2 * n - 1];

        shuffle(&mut permutation);

        let pos = permutation
            .iter()
            .position(|&x| x == mandatory_col)
            .unwrap();

        permutation.swap(pos, mandatory_row);

        let info = |row: usize, col: usize| (n + col - row - 1, row + col);

        for (row, col) in permutation.iter().enumerate() {
            let (main_diag, second_diag) = info(row, *col);

            main_diags[main_diag] += 1;
            second_diags[second_diag] += 1;
        }

        let mut has_swapped = true;

        while has_swapped {
            has_swapped = false;

            for row in 0..n {
                let col = permutation[row];
                let (main_diag, second_diag) = info(row, col);

                if row == mandatory_row {
                    continue;
                }

                for row_2 in row + 1..n {
                    if row_2 == mandatory_row {
                        continue;
                    }

                    let col_2 = permutation[row_2];
                    let (main_diag_2, second_diag_2) = info(row_2, col_2);
                    let (main_diag_new, second_diag_new) = info(row, col_2);
                    let (main_diag_2_new, second_diag_2_new) = info(row_2, col);

                    if num_collisions(&main_diags, main_diag, main_diag_2)
                        + num_collisions(&second_diags, second_diag, second_diag_2)
                        > num_collisions_new(&main_diags, main_diag_new, main_diag_2_new)
                            + num_collisions_new(&second_diags, second_diag_new, second_diag_2_new)
                    {
                        permutation[row] = col_2;
                        permutation[row_2] = col;
                        main_diags[main_diag] -= 1;
                        main_diags[main_diag_new] += 1;
                        second_diags[second_diag] -= 1;
                        second_diags[second_diag_new] += 1;
                        main_diags[main_diag_2] -= 1;
                        main_diags[main_diag_2_new] += 1;
                        second_diags[second_diag_2] -= 1;
                        second_diags[second_diag_2_new] += 1;
                        has_swapped = true;
                        break;
                    }
                }
            }
        }

        if main_diags.iter().chain(second_diags.iter()).all(|&x| x < 2) {
            let mut result: Vec<String> = Vec::with_capacity(n);

            for row in 0..n {
                let mut row_repr = vec!['.'; n];

                row_repr[permutation[row]] = 'Q';
                row_repr.push('\n');

                result.push(row_repr.into_iter().collect());
            }

            return Some(result.into_iter().collect());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::solve_n_queens;

    #[test]
    fn basic_tests() {
        let basic_tests = vec![(1, (0, 0)), (4, (2, 0)), (8, (3, 0))];
        for (n, fixed) in basic_tests.into_iter() {
            test_solution(n, fixed);
        }
    }

    #[test]
    fn no_solution_tests() {
        let no_solutions = vec![(2, (0, 0)), (3, (2, 0)), (6, (0, 0))];
        for (n, fixed) in no_solutions.into_iter() {
            test_no_solution(n, fixed);
        }
    }

    #[test]
    fn big_board() {
        let basic_tests = vec![(20, (3, 0))];
        for (n, fixed) in basic_tests.into_iter() {
            test_solution(n, fixed);
        }
    }

    fn check_board(board: &[u8], n: usize, fixed: (usize, usize)) {
        let mut offset = 0;
        let mut num_queens = 0;
        let mut queens: Vec<Option<usize>> = vec![None; n];
        #[allow(clippy::needless_range_loop)] // should be more clear to keep the `y` indexing
        for y in 0..n {
            for x in 0..n {
                match board[offset] {
                    b'Q' => {
                        assert!(
                            queens[y].is_none(),
                            "The board should not have horizontal attacks between Queens"
                        );
                        num_queens += 1;
                        queens[y] = Some(x);
                    }
                    b'.' => {}
                    _ => panic!("The board has invalid character"),
                }
                offset += 1;
            }

            assert_eq!(
                board[offset], b'\n',
                "The board has missing/incorrect characters"
            );
            offset += 1;
        }

        assert_eq!(
            num_queens, n,
            "The number of queens should be equal to size"
        );

        let queens = queens.into_iter().map(Option::unwrap).collect::<Vec<_>>();
        assert!(
            queens[fixed.1] == fixed.0,
            "The mandatory queen is not in the required position"
        );

        // Check no attacks
        let mut taken_cols = vec![false; n];
        let mut taken_diag1 = vec![false; 2 * n];
        let mut taken_diag2 = vec![false; 2 * n];
        for row in 0..n {
            let col = queens[row];
            assert!(
                !taken_cols[col],
                "The board has vertical attacks between Queens"
            );
            assert!(
                !taken_diag1[col + row],
                "The board has diag1 attacks between Queens"
            );
            assert!(
                !taken_diag2[n + col - row - 1],
                "The board has diag2 attacks between Queens"
            );
            taken_cols[col] = true;
            taken_diag1[col + row] = true;
            taken_diag2[n + col - row - 1] = true;
        }
    }

    fn test_solution(n: usize, fixed: (usize, usize)) {
        if let Some(board) = solve_n_queens(n, fixed) {
            check_board(&board.as_bytes(), n, fixed);
        } else {
            panic!("Returned None when there's a solution");
        }
    }

    fn test_no_solution(n: usize, fixed: (usize, usize)) {
        assert_eq!(
            solve_n_queens(n, fixed),
            None,
            "Expected None when no solution is possible"
        );
    }
}
