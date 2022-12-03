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
    #[serde(rename = "Oracle Table")]
    #[serde(default)]
    pub oracle_table: Vec<OracleTable>,
    #[serde(rename = "Oracles")]
    pub oracles: Option<Vec<Oracle2>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleTable {
    #[serde(rename = "Chance")]
    pub chance: i64,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Prompt")]
    pub prompt: Option<String>,
    #[serde(rename = "Oracle Table")]
    pub oracle_table: Option<Vec<OracleTable2>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleTable2 {
    #[serde(rename = "Chance")]
    pub chance: i64,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Oracle2 {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Oracle Table")]
    pub oracle_table: Vec<OracleTable3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleTable3 {
    #[serde(rename = "Chance")]
    pub chance: i64,
    #[serde(rename = "Description")]
    pub description: String,
}
