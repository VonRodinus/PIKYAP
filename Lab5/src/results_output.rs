//results_output.rs
use log::info;

/// Функция вывода результатов.
pub fn output_results(data: &[i32], average: Option<f64>) {
    info!("Отсортированные данные: {:?}", data);
    match average {
        Some(avg) => info!("Среднее значение: {:.2}", avg),
        None => info!("Невозможно вычислить среднее значение: данные отсутствуют."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;

    #[test]
    fn test_output_results() {
        // Инициализация логгера с использованием env_logger
        env_logger::builder()
            .filter_level(LevelFilter::Info)
            .is_test(true) // Настраиваем для использования в тестах
            .init();

        let data = vec![1, 2, 3, 4, 5];
        let average = Some(3.0);
        output_results(&data, average);
    }
}
