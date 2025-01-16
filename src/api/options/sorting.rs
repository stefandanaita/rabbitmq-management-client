use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RabbitMqSorting {
    #[serde(skip_serializing_if = "Option::is_none", rename = "sort")]
    pub key: Option<String>,
    #[serde(default, rename = "sort_reverse")]
    pub reversed: bool,
}
