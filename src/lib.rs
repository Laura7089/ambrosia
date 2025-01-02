#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]

use std::{cell::LazyCell, collections::HashSet};

use serde::{Deserialize, Serialize};

const DEFAULT_GROUPS_RAW: &[&str] = &[
    include_str!("../data/groups/cheese.toml"),
    include_str!("../data/groups/fish.toml"),
    include_str!("../data/groups/meat.toml"),
    include_str!("../data/groups/shellfish.toml"),
];

const DEFAULT_DIETS_RAW: &[&str] = &[
    include_str!("../data/diets/vegan.toml"),
    include_str!("../data/diets/vegetarian.toml"),
];

/// Default groups baked into the library.
///
#[doc = include_str!("../DISCLAIMER.md")]
pub const DEFAULT_GROUPS: LazyCell<Vec<IngredientGroup>> = LazyCell::new(|| {
    DEFAULT_GROUPS_RAW
        .iter()
        .map(|group_raw| toml::from_str(group_raw).expect("parse error in default group"))
        .collect()
});

/// Default diets baked into the library.
///
#[doc = include_str!("../DISCLAIMER.md")]
pub const DEFAULT_DIETS: LazyCell<Vec<Diet>> = LazyCell::new(|| {
    DEFAULT_DIETS_RAW
        .iter()
        .map(|diet_raw| toml::from_str(diet_raw).expect("parse error in default diet"))
        .collect()
});

/// Unique ingredient.
pub type Ingredient = String;

/// Group of ingredients.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct IngredientGroup {
    ingredients: HashSet<String>,
}

impl IngredientGroup {
    /// Iterate over all ingredients in the group.
    pub fn ingredients(&self) -> impl Iterator<Item = &Ingredient> {
        (&self.ingredients).into_iter()
    }
}

/// Information about a particular diet.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Diet {
    banned_ingredients: HashSet<String>,
}

impl Diet {
    /// Iterate over all banned ingredients in the diet.
    pub fn banned_ingredients(&self) -> impl Iterator<Item = &Ingredient> {
        (&self.banned_ingredients).into_iter()
    }

    /// Merge a group of ingredients into this diet's banlist.
    pub fn merge_group(&mut self, group: &IngredientGroup) {
        for ingred in &group.ingredients {
            self.banned_ingredients.insert(ingred.clone());
        }
    }
}
