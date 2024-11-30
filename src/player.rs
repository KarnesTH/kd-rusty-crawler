use crate::{Item, ItemType};

pub struct Player {
    pub name: String,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub level: i32,
    pub experience: i32,
    pub experience_to_next_level: i32,
    pub inventory: Vec<Item>,
    pub equipped_weapon: Option<Item>,
    pub equipped_armor: Option<Item>,
}

impl Player {
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

    pub fn gain_experience(&mut self, experience: i32) {
        self.experience += experience;

        while self.experience >= self.experience_to_next_level {
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.experience -= self.experience_to_next_level;
        self.experience_to_next_level = (self.experience_to_next_level as f32 * 1.1) as i32;
        self.health += 10;
        self.attack += 2;
        self.defense += 2;
        self.speed += 2;
    }

    pub fn take_damage(&mut self, amount: i32) -> bool {
        let damage = (amount - self.defense).max(1);
        self.health -= damage;
        self.health <= 0
    }

    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

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
