# Ambrosia

A planning system for a group of people with multiple dietary requirements.
Pulls from [mealie](https://mealie.io/) for recipe information.

The package contains both an executable and a library.

The tool examines recipe ingredients against a known relationship of requirements and banned ingredients, and uses fuzzy matching to filter down to a set of acceptable dishes.

## Configuration

No guarantee is provided that this application will provide safe eating options for all specified people.
The quality of results is completely dependent on the quality of recipe transcription, completeness of dietary information, and fuzzy search parameters.

General parameters are read from the environment, command line and config file.
- Mealie address
- Mealie API key
- Fuzzy matching threshold

### People

Peoples' requirements are specified in the `people/` folder.

Needs can be specified in diverse ways:

```toml
[people.john]
name = "John"

diets = ["vegetarian", "gluten-free"]
banned_ingredients = ["tomato", "mushroom"]
```

### Diets

Diets and requirement aliases (such as "vegan", "vegetarian", "lactose-intolerant" etc) are specified in the `diets/` folder.
Common defaults are provided (see [the folder](./diets_default)).

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
```
