/*
    Appellation: sample <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::ItemId;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "snake_case")]
pub struct SampleModel {
    pub id: ItemId,
    pub name: String,
}

impl SampleModel {
    pub fn new(id: ItemId, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> ItemId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}