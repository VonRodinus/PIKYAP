//data_generation.rs
use rand::Rng;

/// Функция генерации случайных чисел.
/// Принимает количество элементов и максимальное значение.
pub fn generate_random_data(count: usize, max_value: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| rng.gen_range(0..max_value)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_data() {
        let data = generate_random_data(10, 50);
        assert_eq!(data.len(), 10);
        assert!(data.iter().all(|&x| x < 50));
    }
}