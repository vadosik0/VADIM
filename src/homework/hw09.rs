pub fn rotate2(input: String, offset: isize) -> String {
    let length = input.chars().count();
    if length == 0 {
        return input;
    }

    let normalized_shift = (offset.rem_euclid(length as isize)) as usize;
    if normalized_shift == 0 {
        return input;
    }

    let characters: Vec<char> = input.chars().collect();
    let split_index = length - normalized_shift;

    let mut result = String::with_capacity(input.len());
    result.extend(characters[split_index..].iter());
    result.extend(characters[..split_index].iter());
    result
}

#[cfg(test)]
mod tests {
    use super::rotate2;

    #[test]
    fn rotate2_test_cases() {
        let original = "abcdefgh";
        let test_cases = [
            (0,    "abcdefgh"),
            (8,    "abcdefgh"),
            (-8,   "abcdefgh"),
            (1,    "habcdefg"),
            (2,    "ghabcdef"),
            (10,   "ghabcdef"),
            (-1,   "bcdefgha"),
            (-2,   "cdefghab"),
            (-10,  "cdefghab"),
        ];

        for (shift, expected) in test_cases.iter() {
            assert_eq!(rotate2(original.to_string(), *shift), expected.to_string());
        }
    }
}
