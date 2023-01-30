pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        Some(&s) => {
            list.windows(2)
                .map(|win| format!("For want of a {} the {} was lost.\n", win[0], win[1]))
                .chain([format!("And all for the want of a {}.", s)])
                .collect()
        }
        None => String::new()
    }
}
