use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProductType {
    Spice,
    Minerals,
    Biomatter,
}

impl ProductType {
    pub fn name(&self) -> &str {
        match self {
            ProductType::Spice => "Spice",
            ProductType::Minerals => "Minerals",
            ProductType::Biomatter => "Biomatter",
        }
    }

    pub fn all() -> Vec<ProductType> {
        vec![ProductType::Spice, ProductType::Minerals, ProductType::Biomatter]
    }
}

// === ECONOMY CONSTANTS ===
pub const BUY_PRODUCT_PRICE:  i32 = 20;
pub const SELL_PRODUCT_PRICE: i32 = 23;
pub const FUEL_PRICE: i32 = 5;
pub const FOOD_PRICE: i32 = 3;

// === GAMEPLAY TUNING CONSTANTS ===
// Starting resources
pub const STARTING_FUEL: f32 = 50.0;
pub const STARTING_FOOD: i32 = 30;
pub const STARTING_MONEY: i32 = 100;

// Consumption rates
pub const FUEL_CONSUMPTION_PER_SECOND: f32 = 0.5;  // Fuel consumed per second while moving
pub const FOOD_CONSUMPTION_INTERVAL: f32 = 5.0;    // Seconds between food consumption
pub const FOOD_CONSUMED_PER_INTERVAL: i32 = 1;     // Food consumed per interval

// Buying amounts
pub const FUEL_BUY_AMOUNT: f32 = 10.0;             // Fuel units received per purchase

#[derive(Debug, Clone)]
pub struct Inventory {
    pub cargo: HashMap<ProductType, i32>,
    pub fuel: f32,
    pub food: i32,
    pub money: i32,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            cargo: HashMap::new(),
            fuel: STARTING_FUEL,
            food: STARTING_FOOD,
            money: STARTING_MONEY,
        }
    }

    pub fn total_cargo(&self) -> i32 {
        self.cargo.values().sum::<i32>() + self.fuel.ceil() as i32 + self.food
    }

    pub fn available_space(&self) -> i32 {
        100 - self.total_cargo()
    }

    pub fn add_cargo(&mut self, product: ProductType, amount: i32) -> bool {
        if self.available_space() >= amount {
            *self.cargo.entry(product).or_insert(0) += amount;
            true
        } else {
            false
        }
    }

    pub fn remove_cargo(&mut self, product: ProductType, amount: i32) -> bool {
        if let Some(current) = self.cargo.get_mut(&product) {
            if *current >= amount {
                *current -= amount;
                if *current == 0 {
                    self.cargo.remove(&product);
                }
                return true;
            }
        }
        false
    }

    pub fn sell_all_cargo(&mut self, planet_product: ProductType) -> i32 {
        let mut total_earned = 0;

        // Sell all cargo except the planet's own product
        let products_to_sell: Vec<ProductType> = self.cargo.keys()
            .filter(|&&p| p != planet_product)
            .copied()
            .collect();

        for product in products_to_sell {
            if let Some(amount) = self.cargo.remove(&product) {
                total_earned += amount * SELL_PRODUCT_PRICE;
            }
        }

        self.money += total_earned;
        total_earned
    }
}
