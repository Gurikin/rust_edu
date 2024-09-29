use crate::devices::Device;

use std::collections::HashMap;

pub struct Space {
    name: String,
    devices: HashMap<String, Device>,
}