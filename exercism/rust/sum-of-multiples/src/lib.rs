pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut filled = vec![false; limit as usize];

    for &factor in factors {
        if factor == 0 {
            continue;
        }

        for x in (factor..limit).into_iter().step_by(factor as usize) {
            filled[x as usize] = true;
        }
    }

    filled
        .into_iter()
        .enumerate()
        .map(|(index, fillment)| if fillment { index as u32 } else { 0u32 })
        .sum()
}
