#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]

use std::collections::HashSet;

use delegate::delegate;
use serde::Deserialize;

mod parse;

/// Unique ingredient.
pub type Ingredient = String;

pub use parse::{DEFAULT_DIETS, DEFAULT_GROUPS};

/// Group of ingredients.
///
/// ```
/// use ambrosia::*;
///
/// let mut my_group = IngredientGroup::default();
/// my_group.merge_ingredient("camembert".to_owned());
/// for ingred in my_group.ingredients() {
///     println!("{ingred}");
/// }
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct IngredientGroup {
    #[serde(default)]
    ingredients: HashSet<String>,
}

impl IngredientGroup {
    /// Merge contents of another group into `self`.
    pub fn merge_group(&mut self, other: &Self) {
        for ingred in &other.ingredients {
            self.merge_ingredient(ingred.clone());
        }
    }

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
    #[serde(default)]
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
