/* Лабораторная работа №2
Цель:

Написать программу, которая выполняет несколько математических операций: вычисление факториала числа, проверка числа на простоту и нахождение всех простых чисел в диапазоне от 1 до N (где N — число, вводимое пользователем).
Программа должна демонстрировать использование циклов, управляющих конструкций и функций.

Задание:

Запросить у пользователя целое число 𝑁.

Реализовать функцию для вычисления факториала числа N.

Реализовать функцию для проверки, является ли число простым.

Найти все простые числа в диапазоне от 1 до 𝑁 с помощью функции проверки простоты числа.

Для каждого простого числа вычислить его факториал и вывести результаты на экран.
*/




use std::io;

//Функция для вычисления факториала числа
fn factorial(num: u32) -> u64 {
    let mut result: u64 = 1;
    for i in 1..=num {
        result *= i as u64;
    }
    result
}

//Функция для проверки числа на простоту
fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u32) {
        if num % i == 0 {
            return false;
        }
    }
    true
}



//Основная функция программы
fn main() {

    //Запрос у пользователя числа n
    println!("Введите число N: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");
    let n: u32 = input.trim().parse().expect("Не удалось преобразовать в число");
    
    
    //Нахождение факториала числа n
    let factorial_n = factorial(n);
    println!("Факториал числа {}: {}", n, factorial_n);


    //Проверка числа n на простоту
    if is_prime(n) {
        println!("Число {} является простым.", n);
    } else {
        println!("Число {} не является простым.", n);
    }


    //Нахождение всех простых чисел в диапазоне от 1 до n
    println!("Простые числа в диапазоне от 1 до {}:", n);
    for i in 1..=n {
        if is_prime(i) {
            let fact = factorial(i);
            println!("{}, факториал: {}", i, fact);
        }
    }
}
