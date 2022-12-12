pub fn dup_to_val(dup: char) -> usize {
    let mut low_alph: Vec<char> = ('a'..='z').collect();
    let mut upp_alph: Vec<char> = ('A'..='Z').collect();
    low_alph.append(&mut upp_alph);
    let val = low_alph.iter().position(|&r| r == dup).unwrap() + 1;
    val
}
