use std::iter;

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut count = size as isize;
    let mut num = 1;
    let mut vec: Vec<Vec<u32>> = iter::repeat(iter::repeat(0).take(size as usize).collect())
        .take(size as usize)
        .collect();
    while count > 0 {
        if count == 1 {
            vec[y][x] = num;
        } else {
            let count = count as usize;
            (x..x + count - 1).for_each(|x| {
                vec[y][x] = num;
                num += 1;
            });
            (y..y + count - 1).for_each(|y| {
                vec[y][x + count - 1] = num;
                num += 1;
            });
            (x + 1..x + count).rev().for_each(|x| {
                vec[y + count - 1][x] = num;
                num += 1;
            });
            (y + 1..y + count).rev().for_each(|y| {
                vec[y][x] = num;
                num += 1;
            });
        }
        x += 1;
        y += 1;
        count -= 2;
    }
    vec
}
