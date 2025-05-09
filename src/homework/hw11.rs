use rand::Rng;

pub fn run() {
    let mut rng = rand::thread_rng();
    let values: Vec<i32> = (0..20).map(|_| rng.gen_range(10..=100)).collect();

    let index_line = (0..20)
        .map(|i| format!("{:>2}.", i))
        .collect::<Vec<_>>()
        .join("  ");

    println!("indexes: {}", index_line);
    println!("data:   {:?}", values);

    if values.len() < 2 {
        return;
    }

    let mut smallest_sum = i32::MAX;
    let mut smallest_index = 0;

    for i in 0..values.len() - 1 {
        let current_sum = values[i] + values[i + 1];
        if current_sum < smallest_sum {
            smallest_sum = current_sum;
            smallest_index = i;
        }
    }

    let left_spacing = " ".repeat(smallest_index * 4);
    let right_spacing = " ".repeat((values.len() - smallest_index - 2) * 4);

    println!("indexes: {}\\__ __/{}", left_spacing, right_spacing);
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        values[smallest_index],
        values[smallest_index + 1],
        smallest_sum,
        smallest_index,
        smallest_index + 1
    );
}
