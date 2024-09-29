use crate::smart_house_mod::space::Space;

use std::collections::HashMap;

pub struct SmartHouse {
    name: String,
    spaces: HashMap<String, Space>,
}