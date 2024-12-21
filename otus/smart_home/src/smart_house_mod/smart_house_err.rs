#[derive(Debug, thiserror::Error)]
pub enum ApartmentError {
    #[error("Ошибка добавления устройства {0} в помещение {1}. Такое устройство уже есть.")]
    AddError(String, String),
    #[error("Ошибка удаления устройства {0} из помещения {1}. Устройство не найдено.")]
    RemoveError(String, String),
}

#[derive(Debug, thiserror::Error)]
pub enum SmartHouseError {
    #[error(
        "Ошибка добавления помещения {0} в умный дом {1}. Помещение с таким имененем уже есть."
    )]
    Add(String, String),
    #[error("Ошибка удаления помещения {0} из умного дома {1}. Помещение не найдено.")]
    Remove(String, String),
    #[error("Устройство {0} не найдено в умном доме {1}. Проверьте наличие умного устройства в Вашем провайдере.")]
    GetReport(String, String),
}
