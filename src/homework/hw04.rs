fn main() {
    const N: usize = 6; 
    let mut diamond = String::new();

    for i in 1..=N {
    let spaces = " ".repeat(N - i);
    let stars = "*".repeat(2 * i - 1);
    let line = format!("{}{}\n", spaces, stars);
    diamond.push_str(&line);
    }

    for i in (1..N).rev() {
    let spaces = " ".repeat(N - i);
    let stars = "*".repeat(2 * i - 1);
    let line = format!("{}{}\n", spaces, stars);
    diamond.push_str(&line);
    }

    print!("{}", diamond);
}
