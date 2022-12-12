pub fn find_duplicate(vecs: (&Vec<char>, &Vec<char>)) -> Result<char, bool> {
    for i in vecs.0 {
        if vecs.1.contains(&i) {
            return Ok(*i);
        } else {
            continue
        }
    }
    Err(false)
}
