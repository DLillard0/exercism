use std::collections::BTreeMap;

pub fn tally(match_results: &str) -> String {
    let mut map: BTreeMap<&str, String> = BTreeMap::new();
    match_results.split('\n').for_each(|str| {
        if let Some(&[a, b, res]) = str.split(';').collect::<Vec<&str>>().windows(3).next() {
            let mut a_char = 'D';
            let mut b_char = 'D';
            match res {
                "win" => {
                    a_char = 'W';
                    b_char = 'L';
                }
                "loss" => {
                    a_char = 'L';
                    b_char = 'W';
                }
                _ => (),
            }
            map.entry(a)
                .and_modify(|s| s.push(a_char))
                .or_insert(String::from(a_char));
            map.entry(b)
                .and_modify(|s| s.push(b_char))
                .or_insert(String::from(b_char));
        }
    });
    let mut res = map.keys()
        .map(|&k| {
            let str = map.get(k).unwrap();
            let mp = str.len();
            let mut w = 0;
            let mut l = 0;
            let mut d = 0;
            let mut p = 0;
            let empty: String = std::iter::repeat(' ').take(31 - k.len()).collect();
            str.chars().for_each(|c| match c {
                'W' => {
                    w += 1;
                    p += 3;
                }
                'D' => {
                    d += 1;
                    p += 1;
                }
                'L' => l += 1,
                _ => panic!("error"),
            });
            (p, format!(
                "{}{}|  {} |  {} |  {} |  {} |  {}",
                k, empty, mp, w, d, l, p
            ))
        })
        .collect::<Vec<(_, _)>>();
    res.sort_by(|(a, _), (b, _)| b.cmp(a));
    res.insert(0, (0, String::from("Team                           | MP |  W |  D |  L |  P")));
    res.into_iter().map(|(_, s)| s).collect::<Vec<_>>().join("\n")
}
