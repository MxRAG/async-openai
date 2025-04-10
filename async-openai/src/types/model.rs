use serde::{Deserialize, Serialize};

/// Describes an OpenAI model offering that can be used with the API.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Model {
    /// The model identifier, which can be referenced in the API endpoints.
    #[serde(default)]
    pub id: String,
    /// The object type, which is always "model".
    #[serde(default)]
    pub object: String,
    /// The Unix timestamp (in seconds) when the model was created.
    #[serde(default)]
    pub created: u32,
    /// The organization that owns the model.
    #[serde(default)]
    pub owned_by: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListModelResponse {
    #[serde(default)]
    pub object: String,
    #[serde(default)]
    pub data: Vec<Model>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DeleteModelResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub object: String,
    #[serde(default)]
    pub deleted: bool,
}
