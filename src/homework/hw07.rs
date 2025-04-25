fn invert_the_case(s: String) -> String {
    s.chars()
        .flat_map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<Vec<_>>()
            } else if c.is_uppercase() {
                c.to_lowercase().collect::<Vec<_>>()
            } else {
                vec![c]
            }
        })
        .collect()
}

#[test]
fn test_invert_the_case() {
    let data = [
        ("Hello",  "HELLO"),
        ("Привет", "ПРИВЕТ"),
    ];

    for &(input, expected) in &data {
        assert_eq!(invert_the_case(input.to_string()), expected.to_string());
        assert_eq!(invert_the_case(expected.to_string()), input.to_string());
    }
}
