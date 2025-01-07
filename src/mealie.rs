use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

const MAX_RECIPES: usize = 100;
const USER_AGENT: &str = "Ambrosia 0.1.0";

#[derive(Debug)]
pub struct MealieApi {
    url: Url,
    client: Client,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error requesting mealie API: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("recipe slug '{0}' is not valid to use in URL path")]
    BadSlug(String),
    #[error("error deserializing returned data into expected schema: {0}")]
    DeserializationError(#[from] serde_json::Error),
}

impl MealieApi {
    pub fn new(address: &str, api_token: &[u8]) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_bytes(api_token).expect("invalid API token"),
        );

        let client = ClientBuilder::new()
            .default_headers(headers)
            .user_agent(USER_AGENT)
            .build()
            .expect("HTTP client build failure");

        let url = Url::parse(&address).expect("URL parse error");

        Self { url, client }
    }

    pub fn get_recipes(&self) -> Result<Vec<Recipe>, Error> {
        const ENDPOINT: &str = "/api/recipes";

        let url = self.url.join(ENDPOINT).unwrap();
        let resp: RecipesPaged = serde_json::from_str(
            &self
                .client
                .get(url.clone())
                .body(
                    json!({
                        "page": 1,
                        "perPage": MAX_RECIPES,
                    })
                    .to_string(),
                )
                .send()?
                .text()?,
        )?;

        let mut recipes = resp.items;

        for page in 2..=resp.total_pages {
            let page_resp: RecipesPaged = serde_json::from_str(
                &self
                    .client
                    .get(url.clone())
                    .body(
                        json!({
                            "page": page,
                            "perPage": MAX_RECIPES,
                        })
                        .to_string(),
                    )
                    .send()?
                    .text()?,
            )?;
            recipes.extend_from_slice(&page_resp.items);
        }

        recipes
            .into_iter()
            .map(|RecipeSummary { slug, .. }| {
                let url = self
                    .url
                    .join(ENDPOINT)
                    .expect("static url join failure")
                    .join(&slug)
                    .map_err(|_| Error::BadSlug(slug.clone()))?;

                Ok(serde_json::from_str(&self.client.get(url).send()?.text()?)?)
            })
            .collect()
    }
}

#[derive(Deserialize)]
struct RecipesPaged {
    page: usize,
    per_page: usize,
    total: usize,
    total_pages: usize,
    next: Option<String>,
    previous: Option<String>,
    items: Vec<RecipeSummary>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
// #[allow(dead_code)]
#[non_exhaustive]
pub struct RecipeSummary {
    id: Option<String>,
    user_id: Uuid,
    household_id: Uuid,
    group_id: Uuid,
    name: Option<String>,
    slug: String,
    // TODO: add image
    recipe_servings: usize,
    recipe_yield_quantity: usize,
    recipe_yield: Option<String>,
    total_time: Option<String>,
    prep_time: Option<String>,
    cook_time: Option<String>,
    perform_time: Option<String>,
    description: Option<String>,
    recipe_category: Option<Vec<RecipeCategory>>,
    tags: Option<Vec<RecipeTag>>,
    // TODO: add tools
    rating: Option<usize>,
    #[serde(rename = "orgURL")]
    org_url: Option<String>,
    date_added: Option<String>,
    date_updated: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    last_made: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    #[serde(flatten)]
    summary: RecipeSummary,
    #[serde(rename = "recipeIngredient")]
    recipe_ingredients: Vec<RecipeIngredient>,
    // TODO: other fields
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct RecipeIngredient {
    quantity: Option<usize>,
    // TODO: units
    food: Option<RecipeFood>,
    note: Option<String>,
    is_food: bool,
    disable_amount: bool,
    display: String,
    title: Option<String>,
    original_text: Option<String>,
    reference_id: Uuid,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
enum RecipeFood {
    IngredientFood {
        id: Uuid,
        name: String,
        // TODO: other fields
    },
    CreateIngredientFood {
        id: Option<String>,
        name: String,
    },
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct RecipeCategory {
    id: Option<String>,
    name: String,
    slug: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct RecipeTag {
    id: Option<String>,
    name: String,
    slug: String,
}
