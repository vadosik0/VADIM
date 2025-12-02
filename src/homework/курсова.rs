use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    println!("Утиліта підрахунку слів у файлі");

    println!("Введіть шлях до файлу:");
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim();

    let file = File::open(path).expect("Не вдалося відкрити файл");
    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;
    let mut frequency: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        line_count += 1;
        char_count += line.len();

        for word in line.split_whitespace() {
            word_count += 1;
            let word = word.to_lowercase();
            *frequency.entry(word).or_insert(0) += 1;
        }
    }

    println!("Кількість рядків: {}", line_count);
    println!("Кількість слів: {}", word_count);
    println!("Кількість символів: {}", char_count);

    if let Some((most_common, count)) = frequency.iter().max_by_key(|entry| entry.1) {
        println!("Найчастіше слово: '{}' ({} разів)", most_common, count);
    } else {
        println!("Файл порожній або не містить тексту");
    }
}