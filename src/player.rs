//! Player module containing player character stats and inventory management.

use crate::{Item, ItemType};

/// Represents the player character and their attributes.
#[derive(Debug)]
pub struct Player {
    /// Player's name
    pub name: String,
    /// Current health points
    pub health: i32,
    /// Base attack power + equipment bonus
    pub attack: i32,
    /// Base defense + equipment bonus
    pub defense: i32,
    /// Movement and action speed
    pub speed: i32,
    /// Current level
    pub level: i32,
    /// Current experience points
    pub experience: i32,
    /// Experience needed for next level
    pub experience_to_next_level: i32,
    /// Player's inventory of items
    pub inventory: Vec<Item>,
    /// Currently equipped weapon
    pub equipped_weapon: Option<Item>,
    /// Currently equipped armor
    pub equipped_armor: Option<Item>,
}

impl Player {
    /// Creates a new player character.
    ///
    /// # Arguments
    /// * `name` - The name of the player character
    ///
    /// # Returns
    /// A new Player instance with default starting stats
    pub fn new(name: String) -> Self {
        Player {
            name,
            health: 100,
            attack: 10,
            defense: 10,
            speed: 10,
            level: 1,
            experience: 0,
            experience_to_next_level: 100,
            inventory: Vec::new(),
            equipped_weapon: None,
            equipped_armor: None,
        }
    }

    /// Adds experience points and handles level ups.
    ///
    /// # Arguments
    /// * `experience` - Amount of experience points to add
    pub fn gain_experience(&mut self, experience: i32) {
        self.experience += experience;

        while self.experience >= self.experience_to_next_level {
            self.level_up();
        }
    }

    /// Increases player level and stats.
    /// Called automatically by gain_experience when enough XP is accumulated.
    fn level_up(&mut self) {
        self.level += 1;
        self.experience -= self.experience_to_next_level;
        self.experience_to_next_level = (self.experience_to_next_level as f32 * 1.1) as i32;
        self.health += 10;
        self.attack += 2;
        self.defense += 2;
        self.speed += 2;
    }

    /// Applies damage to the player.
    ///
    /// # Arguments
    /// * `amount` - Raw damage amount before defense calculation
    ///
    /// # Returns
    /// `true` if the player dies from this damage, `false` otherwise
    pub fn take_damage(&mut self, amount: i32) -> bool {
        let damage = (amount - self.defense).max(1);
        self.health -= damage;
        self.health <= 0
    }

    /// Heals the player.
    ///
    /// # Arguments
    /// * `amount` - Amount of health to restore
    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
    }

    /// Checks if the player is alive.
    ///
    /// # Returns
    /// `true` if health is above 0, `false` otherwise
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Equips an item from the inventory.
    ///
    /// # Arguments
    /// * `inventory_index` - Index of the item in the inventory to equip
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(String)` with error message if failed
    pub fn add_item(&mut self, inventory_index: usize) -> Result<(), String> {
        if inventory_index >= self.inventory.len() {
            return Err("Invalid Inventory-Index".to_string());
        }

        let item = self.inventory[inventory_index].clone();
        match item.item_type {
            ItemType::Weapon => {
                if let Some(old_weapon) = self.equipped_weapon.take() {
                    self.inventory.push(old_weapon);
                }
                self.equipped_weapon = Some(item);
                self.inventory.remove(inventory_index);
                self.update_stats();
                Ok(())
            }
            ItemType::Armor => {
                if let Some(old_armor) = self.equipped_armor.take() {
                    self.inventory.push(old_armor);
                }
                self.equipped_armor = Some(item);
                self.inventory.remove(inventory_index);
                self.update_stats();
                Ok(())
            }
            _ => Err("This item cannot be used".to_string()),
        }
    }

    /// Uses a consumable item from the inventory.
    ///
    /// # Arguments
    /// * `inventory_index` - Index of the item in the inventory to use
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(String)` with error message if failed
    pub fn use_item(&mut self, inventory_index: usize) -> Result<(), String> {
        if inventory_index >= self.inventory.len() {
            return Err("Invalid Inventory-Index".to_string());
        }

        let item = &self.inventory[inventory_index];
        match item.item_type {
            ItemType::Potion => {
                self.health += item.value;
                self.inventory.remove(inventory_index);
                Ok(())
            }
            _ => Err("This Item cannot be used.".to_string()),
        }
    }

    /// Updates player stats based on equipped items.
    /// Called automatically when equipment changes.
    fn update_stats(&mut self) {
        self.attack = 10 + (self.level - 1) * 2;

        if let Some(weapon) = &self.equipped_weapon {
            self.attack += weapon.value;
        }

        self.defense = 10 + (self.level - 1) * 2;

        if let Some(armor) = &self.equipped_armor {
            self.defense += armor.value;
        }
    }
}
