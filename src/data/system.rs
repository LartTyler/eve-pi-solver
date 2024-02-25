use crate::items::ItemId;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct System {
    pub label: String,
    pub planets: Vec<Planet>,
}

#[derive(Debug, Deserialize)]
pub struct Planet {
    pub label: String,
    pub type_label: String,
    pub owned: bool,
    pub tax_rate: f32,
    pub resources: HashMap<ItemId, f32>,
}
