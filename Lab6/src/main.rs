/*
Цель: Реализовать более сложные задачи с использованием замыканий и итераторов, работая с вектором структур, содержащих имя и возраст человека.

Задание
Разработать программу, которая:

Создает вектор структур, представляющих людей (с полями name и age).
Использует замыкания для фильтрации людей старше определенного возраста и вывода их на экран.
Применяет итераторы для выполнения нескольких операций: сортировки, фильтрации и преобразования данных (например, превращение имени в верхний регистр).
Реализует сложный итератор, который работает с кортежами, где каждый кортеж состоит из имени и возраста.
Напишет функцию, которая возвращает новый итератор с замыканием для выполнения кастомной обработки элементов.
*/


// Определяем структуру Person
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// Функция для создания вектора людей
fn create_people() -> Vec<Person> {
    vec![
        Person {
            name: String::from("Alice"),
            age: 30,
        },
        Person {
            name: String::from("Bob"),
            age: 25,
        },
        Person {
            name: String::from("Charlie"),
            age: 35,
        },
        Person {
            name: String::from("Diana"),
            age: 22,
        },
    ]
}

// Функция для фильтрации и обработки людей
fn filter_and_process(people: &[Person], min_age: u32) {
    let filtered: Vec<String> = people
        .iter()
        .filter(|p| p.age > min_age) // Фильтруем по возрасту
        .map(|p| p.name.to_uppercase()) // Превращаем имена в верхний регистр
        .collect();

    println!("Люди старше {} лет: {:?}", min_age, filtered);
}

// Определяем итератор, работающий с кортежами
fn custom_iterator(people: Vec<Person>) -> impl Iterator<Item = (String, u32)> {
    people.into_iter().map(|p| (p.name, p.age))
}

// Основная функция
fn main() {
    let people = create_people();

    // Фильтруем и обрабатываем
    filter_and_process(&people, 25);

    // Используем кастомный итератор
    let person_tuples: Vec<(String, u32)> = custom_iterator(people).collect();
    println!("Кортежи (имя, возраст): {:?}", person_tuples);
}

