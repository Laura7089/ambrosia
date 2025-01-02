#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]

use std::{
    cell::LazyCell,
    collections::{HashMap, HashSet},
};

use delegate::delegate;
use serde::Deserialize;

const DEFAULT_GROUPS_RAW: &[&str] = &[
    include_str!("../data/groups/cheese.toml"),
    include_str!("../data/groups/fish.toml"),
    include_str!("../data/groups/meat.toml"),
    include_str!("../data/groups/shellfish.toml"),
    include_str!("../data/groups/dairy.toml"),
];

/// Unique ingredient.
pub type Ingredient = String;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
struct IngredientsGroupUnflattened {
    #[serde(flatten)]
    inner: IngredientGroup,
    #[serde(default)]
    subgroups: Vec<String>,
}
type IngredientsGroupsFreshParse = HashMap<String, IngredientsGroupUnflattened>;

fn flatten_groups(
    groups_unflattened: IngredientsGroupsFreshParse,
) -> HashMap<String, IngredientGroup> {
    let mut map = HashMap::new();

    for (name, group_unflattened) in &groups_unflattened {
        let mut group = group_unflattened.inner.clone();
        for subgroup in &group_unflattened.subgroups {
            group.merge_group(
                &groups_unflattened
                    .get(subgroup)
                    .unwrap_or_else(|| panic!("unknown subgroup '{subgroup}' referenced"))
                    .inner,
            );
        }
        map.insert(name.clone(), group);
    }

    map
}

/// Default groups baked into the library.
///
#[doc = include_str!("../DISCLAIMER.md")]
pub const DEFAULT_GROUPS: LazyCell<HashMap<String, IngredientGroup>> = LazyCell::new(|| {
    let merged = &DEFAULT_GROUPS_RAW.join("\n");
    let raw: HashMap<String, IngredientsGroupsFreshParse> =
        toml::from_str(&merged).expect("parse error in default groups");

    let groups_unflattened = raw
        .get("groups")
        .expect("no 'groups' map found in default groups")
        .clone();

    flatten_groups(groups_unflattened)
});

#[derive(Deserialize, Debug, Clone)]
struct DietUnflattened {
    #[serde(flatten)]
    inner: Diet,
    banned_groups: Vec<String>,
}
type DietsFreshParse = HashMap<String, DietUnflattened>;

fn flatten_diets(
    diets_unflattened: DietsFreshParse,
    groups: &HashMap<String, IngredientGroup>,
) -> HashMap<String, Diet> {
    let mut map = HashMap::new();

    for (name, diet_unflattened) in &diets_unflattened {
        let mut diet = diet_unflattened.inner.clone();
        for group in &diet_unflattened.banned_groups {
            diet.merge_group(
                &groups
                    .get(group)
                    .unwrap_or_else(|| panic!("unknown group '{group}' referenced")),
            );
        }
        map.insert(name.clone(), diet);
    }

    map
}

const DEFAULT_DIETS_RAW: &[&str] = &[
    include_str!("../data/diets/vegan.toml"),
    include_str!("../data/diets/vegetarian.toml"),
];

/// Default diets baked into the library.
///
#[doc = include_str!("../DISCLAIMER.md")]
pub const DEFAULT_DIETS: LazyCell<HashMap<String, Diet>> = LazyCell::new(|| {
    let merged = &DEFAULT_DIETS_RAW.join("\n");
    let raw: HashMap<String, DietsFreshParse> =
        toml::from_str(&merged).expect("parse error in default groups");

    let diets_unflattened = raw
        .get("diets")
        .expect("no 'diets' map found in default diets")
        .clone();

    flatten_diets(diets_unflattened, &DEFAULT_GROUPS)
});

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_unflattened_from_toml() {
        let raw = r#"ingredients = ["first", "second"]
subgroups=["other"]"#;
        let _group: IngredientsGroupUnflattened = toml::from_str(raw).expect("parse error");
    }
}
