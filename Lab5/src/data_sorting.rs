//data_sorting.rs

/// Функция сортировки данных.
/// Принимает вектор целых чисел и сортирует его по возрастанию.
pub fn sort_data(data: &mut [i32]) {
    data.sort_unstable();
}

mod tests {
    #[test]
    fn test_sort_data() {
        let mut data = vec![3, 1, 4, 1, 5, 9];
        super::sort_data(&mut data);
        assert_eq!(data, vec![1, 1, 3, 4, 5, 9]);
    }
}