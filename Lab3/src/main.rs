/* Лабораторная работа №3
Цель: 

Написать программу, которая обрабатывает текст, подсчитывает количество слов, букв и символов в предложении.

Задание:

Запросить у пользователя предложение (строку).
Вывести общее количество символов в предложении.
Подсчитать количество букв (игнорируя пробелы и знаки пунктуации).
Разделить строку на слова и вывести количество слов.
*/


use std::collections::HashMap;
use std::io;

fn main() {
    // Запрашиваем предложение у пользователя
    println!("Введите предложение: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");
    let sentence = input.trim();

    // Выводим общее количество символов (включая пробелы и знаки препинания)
    let total_chars = sentence.chars().count();
    println!("Общее количество символов: {}", total_chars);

    // Подсчитываем количество букв (игнорируя пробелы и знаки пунктуации)
    let letters_only: String = sentence.chars().filter(|c| c.is_alphabetic()).collect();
    let total_letters = letters_only.chars().count();
    println!("Количество букв: {}", total_letters);

    // Подсчитываем количество слов
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let word_count = words.len();
    println!("Количество слов: {}", word_count);

    // Подсчитываем частоту появления каждой буквы
    let mut letter_frequency: HashMap<char, usize> = HashMap::new();
    for letter in letters_only.to_lowercase().chars() {
        let counter = letter_frequency.entry(letter).or_insert(0);
        *counter += 1;
    }

    println!("Частота букв:");
    for (letter, count) in letter_frequency {
        println!("{}: {}", letter, count);
    }
}
