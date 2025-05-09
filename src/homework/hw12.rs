use rand::Rng;

fn calculate_balance_steps(values: &Vec<u32>) -> isize {
    let total: u32 = values.iter().sum();
    let len = values.len() as u32;

    if total % len != 0 {
        return -1; // Баланс невозможен
    }

    let average = total / len;
    let mut total_steps = 0;
    let mut balance_offset = 0;

    for &val in values {
        balance_offset += val as i32 - average as i32;
        total_steps += balance_offset.abs() as usize;
    }

    total_steps as isize
}

fn generate_balanced_vector(size: usize) -> Vec<u32> {
    let avg = rand::thread_rng().gen_range(1..100);
    let mut vec = vec![avg; size];

    for i in 0..size / 2 {
        let delta = rand::thread_rng().gen_range(0..=avg.min(10));
        vec[i] += delta;
        vec[size - 1 - i] -= delta;
    }

    vec
}

pub fn main() {
    let example1 = vec![8, 2, 2, 4, 4];
    let result1 = calculate_balance_steps(&example1);
    println!("Example 1: {:?} => Moves: {}", example1, result1);

    let example2 = vec![9, 3, 7, 2, 9];
    let result2 = calculate_balance_steps(&example2);
    println!("Example 2: {:?} => Moves: {}", example2, result2);

    let example3 = vec![1, 1, 1, 1, 6];
    let result3 = calculate_balance_steps(&example3);
    println!("Example 3: {:?} => Moves: {}", example3, result3);

    let generated = generate_balanced_vector(10);
    println!("Generated vector: {:?}", generated);
    let generated_result = calculate_balance_steps(&generated);
    println!("Moves required to balance generated vector: {}", generated_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_one() {
        let data = vec![8, 2, 2, 4, 4];
        assert_eq!(calculate_balance_steps(&data), 6);
    }

    #[test]
    fn test_case_two() {
        let data = vec![9, 3, 7, 2, 9];
        assert_eq!(calculate_balance_steps(&data), 7);
    }
}
