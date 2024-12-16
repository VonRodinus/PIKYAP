// cache.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    data: HashMap<String, String>,
}

impl Cache {
    /// Создает новый пустой кэш
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Загружает кэш из файла
    pub fn load_from_file(file_path: &str) -> Result<Self> {
        let content = fs::read_to_string(file_path).unwrap_or_else(|_| "{}".to_string());
        let cache: Cache = serde_json::from_str(&content).context("Failed to parse cache file")?;
        Ok(cache)
    }

    /// Сохраняет кэш в файл
    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(self).context("Failed to serialize cache")?;
        fs::write(file_path, content).context("Failed to write cache to file")?;
        Ok(())
    }

    /// Извлекает данные из кэша
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// Вставляет данные в кэш
    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
}
