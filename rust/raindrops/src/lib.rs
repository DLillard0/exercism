const SOUNDS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    match SOUNDS
        .into_iter()
        .map(|(v, str)| if n % v == 0 { str } else { "" })
        .collect::<String>()
    {
        res if !res.is_empty() => res,
        _ => n.to_string(),
    }
}
