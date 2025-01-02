#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]

use std::{cell::LazyCell, collections::HashSet};

use delegate::delegate;
use serde::Deserialize;

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
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct IngredientGroup {
    ingredients: HashSet<String>,
}

impl IngredientGroup {
    delegate! {
        to self.ingredients {
            /// Merge an ingredient into this group's list.
            #[call(insert)]
            pub fn merge_ingredient(&mut self, ingred: Ingredient);
        }

        to (&self.ingredients) {
            /// Iterate over all ingredients in the group.
            #[call(into_iter)]
            pub fn ingredients(&self) -> impl Iterator<Item = &Ingredient>;
        }
    }
}

/// Information about a particular diet.
///
/// ```
/// use ambrosia::*;
///
/// let mut my_diet = Diet::default();
/// my_diet.merge_ingredient("brie".to_owned());
/// for ingred in my_diet.banned_ingredients() {
///     println!("{ingred}");
/// }
/// ```
#[derive(Clone, Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Diet {
    banned_ingredients: HashSet<String>,
}

impl Diet {
    /// Merge a group of ingredients into this diet's banlist.
    pub fn merge_group(&mut self, group: &IngredientGroup) {
        for ingred in &group.ingredients {
            self.merge_ingredient(ingred.clone());
        }
    }

    delegate! {
        to self.banned_ingredients {
            /// Merge an ingredient into this diet's banlist.
            #[call(insert)]
            pub fn merge_ingredient(&mut self, ingred: Ingredient);
        }

        to (&self.banned_ingredients) {
            /// Iterate over all banned ingredients in the diet.
            #[call(into_iter)]
            pub fn banned_ingredients(&self) -> impl Iterator<Item = &Ingredient>;
        }
    }
}
