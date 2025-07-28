pub fn fresh_id(i: usize) -> String {
    format!("var_{i}")
}

pub fn bytes_to_int(bytes: &Vec<Option<u8>>) -> i128 {
    bytes
        .iter()
        .enumerate()
        .map(|(i, b)| (b.unwrap_or_default() as i128) << i)
        .sum::<i128>()
}
