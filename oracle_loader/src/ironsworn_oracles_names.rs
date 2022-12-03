use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Oracles")]
    pub oracles: Vec<Oracle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Oracle {
    #[serde(rename = "Name")]
    pub name: String,
    pub d: Option<String>,
    #[serde(rename = "Oracle Table")]
    pub oracle_table: Vec<OracleTable>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleTable {
    #[serde(rename = "Chance")]
    pub chance: i64,
    #[serde(rename = "Description")]
    pub description: String,
}
