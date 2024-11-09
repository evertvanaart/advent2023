pub fn compute_hash(input: &str) -> usize {
    let mut result: usize = 0;

    for c in input.chars() {
        let ascii_value: usize = c as usize;
        result += ascii_value;
        result *= 17;
        result = result % 256;
    }

    result
}
