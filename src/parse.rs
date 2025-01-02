use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::{Diet, IngredientGroup};

const DEFAULT_GROUPS_RAW: &[&str] = &[
    include_str!("../data/groups/cheese.toml"),
    include_str!("../data/groups/fish.toml"),
    include_str!("../data/groups/meat.toml"),
    include_str!("../data/groups/shellfish.toml"),
    include_str!("../data/groups/dairy.toml"),
];

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
struct IngredientsGroupUnflattened {
    #[serde(flatten)]
    inner: IngredientGroup,
    #[serde(default)]
    subgroups: Vec<String>,
}
type IngredientsGroupsFreshParse = HashMap<String, IngredientsGroupUnflattened>;

fn flatten_groups(
    groups_unflattened: &IngredientsGroupsFreshParse,
) -> HashMap<String, IngredientGroup> {
    let mut map = HashMap::new();

    for (name, group_unflattened) in groups_unflattened {
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
#[allow(clippy::missing_panics_doc)]
pub fn default_groups() -> &'static HashMap<String, IngredientGroup> {
    static DEFAULT_GROUPS: OnceLock<HashMap<String, IngredientGroup>> = OnceLock::new();

    DEFAULT_GROUPS.get_or_init(|| {
        let merged = &DEFAULT_GROUPS_RAW.join("\n");
        let raw: HashMap<String, IngredientsGroupsFreshParse> =
            toml::from_str(merged).expect("parse error in default groups");

        let groups_unflattened = raw
            .get("groups")
            .expect("no 'groups' map found in default groups")
            .clone();

        flatten_groups(&groups_unflattened)
    })
}

#[derive(Deserialize, Debug, Clone)]
struct DietUnflattened {
    #[serde(flatten)]
    inner: Diet,
    banned_groups: Vec<String>,
}
type DietsFreshParse = HashMap<String, DietUnflattened>;

fn flatten_diets(
    diets_unflattened: &DietsFreshParse,
    groups: &HashMap<String, IngredientGroup>,
) -> HashMap<String, Diet> {
    let mut map = HashMap::new();

    for (name, diet_unflattened) in diets_unflattened {
        let mut diet = diet_unflattened.inner.clone();
        for group in &diet_unflattened.banned_groups {
            diet.merge_group(
                groups
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
#[allow(clippy::missing_panics_doc)]
pub fn default_diets() -> &'static HashMap<String, Diet> {
    static DEFAULT_DIETS: OnceLock<HashMap<String, Diet>> = OnceLock::new();
    DEFAULT_DIETS.get_or_init(|| {
        let merged = &DEFAULT_DIETS_RAW.join("\n");
        let raw: HashMap<String, DietsFreshParse> =
            toml::from_str(merged).expect("parse error in default groups");

        let diets_unflattened = raw
            .get("diets")
            .expect("no 'diets' map found in default diets")
            .clone();

        flatten_diets(&diets_unflattened, default_groups())
    })
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
