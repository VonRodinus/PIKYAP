//main.rs
/*
Цель: Создать проект с использованием нескольких модулей и внешних библиотек, а также реализовать автоматическое тестирование функций.

Программа должна сгенерировать массив чисел, отсортировать его, рассчитать среднее значение элементов в массиве и вывести каждый из этих этапов в консоль.
*/




mod data_generation;
mod data_sorting;
mod data_processing;
mod results_output;

use log::LevelFilter;
use simple_logger::SimpleLogger;

fn main() {
    // Инициализация логгера
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    // Генерация данных
    let mut data = data_generation::generate_random_data(10, 100);
    log::info!("Сгенерированные данные: {:?}", data);

    // Сортировка данных
    data_sorting::sort_data(&mut data);

    // Обработка данных
    let average = data_processing::calculate_average(&data);

    // Вывод результатов
    results_output::output_results(&data, average);
}

