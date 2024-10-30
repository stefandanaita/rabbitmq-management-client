use serde::Serialize;

#[derive(Debug, Clone)]
pub struct RabbitMqPagination {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub filter: Option<RabbitMqPaginationFilter>,
}

#[derive(Debug, Clone)]
pub enum RabbitMqPaginationFilter {
    StringFilter(String),
    RegexFilter(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct RabbitMqPaginationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_regex: Option<bool>,
    pub pagination: bool,
}

impl From<RabbitMqPagination> for RabbitMqPaginationRequest {
    fn from(value: RabbitMqPagination) -> Self {
        let pagination =
            value.page.is_some() || value.page_size.is_some() || value.filter.is_some();

        let (name, use_regex) = match value.filter {
            None => (None, None),
            Some(f) => match f {
                RabbitMqPaginationFilter::StringFilter(s) => (Some(s), Some(false)),
                RabbitMqPaginationFilter::RegexFilter(r) => (Some(r), Some(true)),
            },
        };

        Self {
            page: value.page,
            page_size: value.page_size,
            name,
            use_regex,
            pagination,
        }
    }
}
