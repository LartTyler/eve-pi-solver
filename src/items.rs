use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::solver::Tier;

#[derive(Debug, thiserror::Error)]
pub enum ItemError {
    #[error("io: {0}")]
    IO(#[from] std::io::Error),

    #[error("deserialize error: {0}")]
    Deserialize(#[from] serde_yaml::Error),
}

pub type ItemId = String;

type ItemMap = HashMap<ItemId, Item>;
type UsedInMap = HashMap<String, Vec<String>>;

pub struct ItemManager {
    items: ItemMap,
    used_in_map: UsedInMap,
}

impl ItemManager {
    pub fn new(data_file: &Path) -> Result<Self, ItemError> {
        let items: ItemMap = serde_yaml::from_str(&fs::read_to_string(data_file)?)?;
        let items: ItemMap = items
            .into_iter()
            .map(|(k, v)| (k.clone(), Item { id: k, ..v }))
            .collect();

        let mut used_in_map = UsedInMap::new();

        for (id, item) in &items {
            let Some(production) = &item.production else {
                continue;
            };

            for input in production.inputs.keys() {
                let mapping = match used_in_map.get_mut(input) {
                    Some(v) => v,
                    None => {
                        used_in_map.insert(input.to_string(), Vec::new());
                        used_in_map.get_mut(input).unwrap()
                    }
                };

                if !mapping.contains(id) {
                    mapping.push(id.to_string());
                }
            }
        }

        Ok(Self { items, used_in_map })
    }

    pub fn get<Id>(&self, item_id: Id) -> Option<&Item>
    where
        Id: AsRef<str>,
    {
        self.items.get(item_id.as_ref())
    }

    pub fn find_products<Id>(&self, item_id: Id) -> Vec<&Item>
    where
        Id: AsRef<str>,
    {
        self.used_in_map
            .get(item_id.as_ref())
            .into_iter()
            .flatten()
            .filter_map(|id| self.get(id))
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(default)]
    pub id: String,
    pub label: String,
    pub tier: Tier,
    pub production: Option<Production>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Production {
    pub inputs: HashMap<ItemId, u16>,
    pub quantity: u8,
}
