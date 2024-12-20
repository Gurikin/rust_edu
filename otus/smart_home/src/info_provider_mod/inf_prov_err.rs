#[derive(Debug, thiserror::Error)]
pub enum DeviceInfoStorageError {
    #[error("Ошибка добавления устройства в провайдер. Комната '{0}' не найдена. Сначала добавьте комнату.")]
    AddError(String),
    #[error("Ошибка удаления устройства из провайдера. Комната '{0}' не найдена. Сначала добавьте комнату.")]
    RemoveError(String)
}