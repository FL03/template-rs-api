/*
    Appellation: sample <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{ItemId, Timestamp};

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    sqlx::FromRow,
)]
#[serde(default, rename_all = "snake_case")]
#[sqlx(default, rename_all = "snake_case")]
pub struct ItemModel {
    pub id: ItemId,
    pub title: String,
    pub description: String,
    pub created_at: Timestamp,
}

impl ItemModel {
    pub fn new(title: impl ToString, description: impl ToString) -> Self {
        let id = uuid::Uuid::new_v4();
        let created_at = chrono::Utc::now();
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            created_at,
        }
    }

    pub fn id(&self) -> ItemId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
