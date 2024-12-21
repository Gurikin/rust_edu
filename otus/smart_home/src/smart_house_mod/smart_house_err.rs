#[derive(Debug, thiserror::Error)]
pub enum ApartmentError {
    #[error("Ошибка добавления устройства {0} в помещение {1}. Такое устройство уже есть.")]
    AddError(String, String),
    #[error("Ошибка удаления устройства {0} из помещения {1}. Устройство не найдено.")]
    RemoveError(String, String)
}