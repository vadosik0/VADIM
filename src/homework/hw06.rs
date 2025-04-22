fn main() {
    const УРОВНИ: usize = 5;
    let mut ёлка = String::new();
    let ширина = 2 * УРОВНИ + 1;

    for уровень in 1..=УРОВНИ {
    for строка in 1..=уровень {
    let звёзды = "*".repeat(2 * строка - 1);
    let пробелы = " ".repeat((ширина - звёзды.len()) / 2);
    ёлка.push_str(&format!("{}{}\n", пробелы, звёзды));
        }
    }

    print!("{}", ёлка);
}
