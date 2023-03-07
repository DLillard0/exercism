pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_max = vec![];
    let mut col_min = vec![];
    let mut res = vec![];
    input.iter().enumerate().for_each(|(i, vec)| {
        vec.iter().enumerate().for_each(|(j, num)| {
            if row_max.len() <= i {
                row_max.push(num);
            } else if row_max[i] < num {
                row_max[i] = num;
            }
            if col_min.len() <= j {
                col_min.push(num);
            } else if col_min[j] > num {
                col_min[j] = num;
            }
        });
    });
    input.iter().enumerate().for_each(|(i, vec)| {
        vec.iter().enumerate().for_each(|(j, num)| {
            if num == row_max[i] && num == col_min[j] {
                res.push((i, j))
            }
        });
    });
    res
}
