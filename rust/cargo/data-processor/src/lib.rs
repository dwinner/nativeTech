//! Библиотека для обработки данных

use serde::{Deserialize, Serialize};

/// Структура для представления данных
#[derive(Serialize, Deserialize, Debug)]
pub struct Data
{
   id: u32,
   value: String,
}

/// Создаёт новый объект данных
pub fn create_data(id: u32, value: &str) -> Data
{
   Data {
      id,
      value: value.to_string(),
   }
}

/// Сериализует данные в JSON
pub fn to_json(data: &Data) -> String
{
   serde_json::to_string(data).unwrap_or_else(|err| format!("Ошибка: {}", err))
}
