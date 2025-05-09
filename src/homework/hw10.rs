pub fn is_palindrome(x: u32) -> bool {
    let original_value = x;
    let mut value = x;
    let mut reversed: u32 = 0;

    while value > 0 {
        let last_digit = value % 10;
        reversed = match reversed
            .checked_mul(10)
            .and_then(|res| res.checked_add(last_digit))
        {
            Some(val) => val,
            None => return false,
        };
        value /= 10;
    }

    reversed == original_value
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn test_palindromes() {
        let test_cases = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        for (input, expected_result) in test_cases.iter() {
            assert_eq!(is_palindrome(*input), *expected_result);
        }
    }
}
