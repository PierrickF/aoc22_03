pub fn split_vec(vec: &Vec<char>) -> (Vec<char>, Vec<char>) {
    let vec1: Vec<char> = vec[0..vec.len() / 2].to_vec();
    let vec2: Vec<char> = vec[vec.len() / 2..vec.len()].to_vec();
    (vec1, vec2)
}
