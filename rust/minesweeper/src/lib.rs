
static NEIGHBORHOOD_OFFSETS: &[(i32, i32)] = &[
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),         (0, 1),
    (1, -1), (1, 0), (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield.iter().enumerate().map(|(i, v)| {
        v.chars().enumerate().map(|(j, c)| {
            match c {
                '*' => '*',
                _ => {
                    let mut num = 0;

                    let m = i as i32;
                    let n = j as i32;

                    NEIGHBORHOOD_OFFSETS.into_iter().for_each(|(a, b)| {
                        if is_mine(minefield, m + a, n + b) { num += 1; }
                    });

                    match num {
                        0 => ' ',
                        _ => char::from_digit(num, 10).unwrap(),
                    }
                },
            }
        }).collect::<String>()
    }).collect()
}

fn is_mine(minefield: &[&str], i: i32, j: i32) -> bool {
    let l = minefield.len();
    if i < 0 || j < 0 || i >= l as i32 {
        return  false;
    }

    match minefield[i as usize].chars().nth(j as usize) {
        Some(s) => s == '*',
        None => false,
    }
}
