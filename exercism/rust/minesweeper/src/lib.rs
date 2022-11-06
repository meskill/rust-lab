use itertools::iproduct;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();

    if height == 0 {
        return vec![];
    }

    let width = minefield[0].len();
    let mut result: Vec<_> = minefield
        .iter()
        .map(|row| row.as_bytes().to_vec())
        .collect();

    for row_index in 0..height {
        for col_index in 0..width {
            let mut output = result[row_index][col_index];

            for (offset_row, offset_col) in iproduct!(-1isize..=1, -1isize..=1) {
                let x = (row_index as isize) + offset_row;
                let y = (col_index as isize) + offset_col;

                if x >= 0 && y >= 0 {
                    let x = x as usize;
                    let y = y as usize;

                    if x < height && y < width && result[x][y] == b'*' {
                        output = match output {
                            b'*' => b'*',
                            b' ' => b'1',
                            v => v + 1,
                        };
                    }
                }
            }

            result[row_index][col_index] = output;
        }
    }

    result
        .into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .collect()
}
