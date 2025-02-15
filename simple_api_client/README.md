# Простой клиент для API

## 1. Введение

### 1.1 Цель проекта
Цель проекта — создать консольное приложение на языке Rust, которое взаимодействует с публичным API (OpenWeatherMap) для получения данных о погоде и отображения их пользователю.

### 1.2 Область применения
Приложение будет использоваться для получения и отображения информации о погоде в заданном городе. Пользователь сможет вводить название города и получать текущую температуру, влажность, скорость ветра и краткое описание погоды.


## 2. Функциональные требования

### 2.1 Основные функции
- **Запрос погоды**: Пользователь вводит название города, приложение отправляет запрос к API и получает данные о погоде.
- **Отображение данных**: Приложение выводит информацию о текущей температуре, влажности, скорости ветра и описании погоды.
- **Обработка ошибок**: Если введено неправильное название города или произошла ошибка при запросе, приложение должно вывести понятное сообщение об ошибке.

## 3. Технические требования

### 3.1 Язык программирования
- Rust (версия 1.60 и выше)

### 3.2 Библиотеки
- `reqwest` — для выполнения HTTP-запросов.
- `serde` и `serde_json` — для сериализации и десериализации JSON-данных.
- `anyhow` — для обработки ошибок и улучшения читаемости кода.

### 3.3 Среда разработки
- Операционная система: любая (Windows, macOS, Linux).
- Компилятор: rustc.
- Утилиты: Cargo для управления зависимостями и сборки.

## 4. Архитектура

### 4.1 Модули
- **main.rs**: 
  - Содержит основную логику приложения, включая инициализацию, обработку ввода и вывод данных.
- **api.rs**: 
  - Реализует функции для выполнения HTTP-запросов к API и обработки ответов, включая обработку ошибок.
- **models.rs**: 
  - Определяет структуры данных для хранения информации о погоде, такие как `WeatherResponse`, `Main`, `Wind` и т.д.
- **cli.rs**: 
  - Обрабатывает ввод командной строки, включая парсинг аргументов и взаимодействие с пользователем.
- **cache.rs**: 
  - Реализует функции для кэширования результатов запросов, включая сохранение и загрузку данных из файла.


## 5. Приложения
- **Ссылки на ресурсы**:
  - [OpenWeatherMap API Documentation](https://openweathermap.org/api)
  - [Rust Documentation](https://doc.rust-lang.org/)
  - [Serde Documentation](https://serde.rs/)
  - [Reqwest Documentation](https://docs.rs/reqwest)
