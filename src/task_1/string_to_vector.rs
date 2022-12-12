pub fn string_to_vector(line: String) -> Vec<char> {
    let mut iter = line.chars();
    let mut vec: Vec<char> = Vec::new();
    for _ in 0..line.chars().count() {
        vec.push(iter.next().unwrap());
    }
    vec
}
