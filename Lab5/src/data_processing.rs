//data_processing.rs

/// Функция вычисления среднего значения.
/// Возвращает среднее значение или None, если массив пуст.
pub fn calculate_average(data: &[i32]) -> Option<f64> {
    if data.is_empty() {
        None
    } else {
        Some(data.iter().map(|&x| x as f64).sum::<f64>() / data.len() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_average() {
        let data = vec![1, 2, 3, 4, 5];
        let average = calculate_average(&data);
        assert_eq!(average, Some(3.0));
    }

    #[test]
    fn test_calculate_average_empty() {
        let data: Vec<i32> = vec![];
        let average = calculate_average(&data);
        assert_eq!(average, None);
    }
}