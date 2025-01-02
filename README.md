# Ambrosia

A meal planning system for a group of people with multiple dietary requirements.
Pulls from [mealie](https://mealie.io/) for recipe information.

The package contains both an executable and a library.

The tool examines recipe ingredients against a known relationship of requirements and banned ingredients, and uses fuzzy matching to filter down to a set of acceptable dishes.

**See the [disclaimer](./DISCLAIMER.md)**.

## Configuration

General parameters are read from the environment, command line and config file.
- Mealie address
- Mealie API key
- Fuzzy matching threshold

### People

Needs can be specified in diverse ways:

```toml
[people.john]
name = "John"

diets = ["vegetarian", "gluten-free"]
banned_ingredients = ["tomato", "mushroom"]
```

### Diets & Groups

Diets here refers to requirement aliases such as "vegan", "vegetarian", "lactose-intolerant" etc.
Common defaults are provided (see [the `data` folder](./data)).

Needs can be specified as so:

```toml
[diets.low_glycaemic_index]

banned_ingredients = ["flour"]

[diets.vegetarian]

banned_groups = ["meat"]

[groups.meat]
ingredients = [
  "beef",
  "pork",
  "chicken",
  # etc...
]

subgroups = [
  "fish"
]
```
