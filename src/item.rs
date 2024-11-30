#[derive(Debug, Clone)]
pub enum ItemType {
    Weapon,
    Armor,
    Potion,
    Key,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub value: i32,
    pub description: String,
}

impl Item {
    pub fn new(name: String, item_type: ItemType, value: i32, description: String) -> Self {
        Item {
            name,
            item_type,
            value,
            description,
        }
    }

    pub fn create_sword() -> Self {
        Item::new(
            "Sword".to_string(),
            ItemType::Weapon,
            10,
            "A simple sword.".to_string(),
        )
    }

    pub fn create_health_potion() -> Self {
        Item::new(
            "Health Potion".to_string(),
            ItemType::Potion,
            20,
            "Restores 20 health.".to_string(),
        )
    }
}
