//! Item system module for managing game items and equipment.

/// Represents different types of items in the game.
#[derive(Debug, Clone)]
pub enum ItemType {
    /// Weapons used for combat
    Weapon,
    /// Armor for defense
    Armor,
    /// Consumable potions
    Potion,
    /// Keys for unlocking doors or chests
    Key,
}

/// Represents an item in the game.
#[derive(Debug, Clone)]
pub struct Item {
    /// Name of the item
    pub name: String,
    /// Type category of the item
    pub item_type: ItemType,
    /// Numerical value (damage for weapons, defense for armor, healing for potions)
    pub value: i32,
    /// Descriptive text about the item
    pub description: String,
}

impl Item {
    /// Creates a new item instance.
    ///
    /// # Arguments
    /// * `name` - The name of the item
    /// * `item_type` - The type category of the item
    /// * `value` - The numerical value of the item's effect
    /// * `description` - A description of the item
    ///
    /// # Returns
    /// A new Item instance with the specified properties
    pub fn new(name: String, item_type: ItemType, value: i32, description: String) -> Self {
        Item {
            name,
            item_type,
            value,
            description,
        }
    }

    /// Creates a basic sword weapon.
    ///
    /// # Returns
    /// A pre-configured sword item with:
    /// * 10 damage value
    /// * Basic description
    pub fn create_sword() -> Self {
        Item::new(
            "Sword".to_string(),
            ItemType::Weapon,
            10,
            "A simple sword.".to_string(),
        )
    }

    /// Creates a health potion.
    ///
    /// # Returns
    /// A pre-configured health potion with:
    /// * 20 healing value
    /// * Basic description
    pub fn create_health_potion() -> Self {
        Item::new(
            "Health Potion".to_string(),
            ItemType::Potion,
            20,
            "Restores 20 health.".to_string(),
        )
    }
}
