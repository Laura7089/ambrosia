#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]

use std::collections::HashSet;

use delegate::delegate;
use nutype::nutype;
use serde::Deserialize;

mod parse;

pub use parse::{DEFAULT_DIETS, DEFAULT_GROUPS};

#[nutype(
    sanitize(lowercase),
    derive(
        Clone,
        Display,
        Debug,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Deserialize,
        From,
        FromStr,
    )
)]
pub struct Ingredient(String);

/// Group of ingredients.
///
/// ```
/// use ambrosia::*;
///
/// let mut my_group = IngredientGroup::default();
/// my_group.merge_ingredient("camembert".into());
/// for ingred in my_group.ingredients() {
///     println!("{ingred}");
/// }
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct IngredientGroup {
    #[serde(default)]
    ingredients: HashSet<Ingredient>,
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

            /// Check if an ingredient is included in the group.
            pub fn contains(&self, ingred: &Ingredient) -> bool;
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
/// my_diet.merge_ingredient("brie".into());
/// for ingred in my_diet.banned_ingredients() {
///     println!("{ingred}");
/// }
/// ```
#[derive(Clone, Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Diet {
    #[serde(default)]
    banned_ingredients: HashSet<Ingredient>,
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

            /// Check if an ingredient is disallowed.
            #[call(contains)]
            pub fn disallows(&self, ingred: &Ingredient) -> bool;
        }

        to (&self.banned_ingredients) {
            /// Iterate over all banned ingredients in the diet.
            #[call(into_iter)]
            pub fn banned_ingredients(&self) -> impl Iterator<Item = &Ingredient>;
        }
    }
}
