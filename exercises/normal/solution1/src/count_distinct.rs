use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let v: Vec<&str> = input_str.split(',').collect();
    let set: HashSet<_> = v.into_iter().collect();
    set.len() as usize
}
