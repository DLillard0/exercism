pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut vec = vec![];
        (0..self.0 as usize).for_each(|r: usize| {
            match r {
                0 => vec.push(vec![1]),
                r => {
                    vec.push(
                        (0..=r).map(|i: usize| {
                            match i {
                                0 => 1,
                                i if i == r => 1,
                                i => {
                                    vec[r - 1][i - 1] + vec[r - 1][i]
                                }
                            }
                        }).collect()
                    )
                }
            };
        });
        vec
    }
}
