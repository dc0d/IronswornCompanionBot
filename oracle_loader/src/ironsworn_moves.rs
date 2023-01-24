use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Source")]
    pub source: Source,
    #[serde(rename = "Categories")]
    pub categories: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Moves")]
    pub moves: Vec<Mfe>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mfe {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Source")]
    pub source: Source2,
    #[serde(rename = "Stats")]
    #[serde(default)]
    pub stats: Vec<String>,
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Progress Move")]
    pub progress_move: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source2 {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Page")]
    pub page: String,
}
