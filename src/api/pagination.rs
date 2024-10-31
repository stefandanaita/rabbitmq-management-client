use serde::{Deserialize, Serialize};

const PAGINATION_PAGE_SIZE: u32 = 50;

#[derive(Debug, Clone)]
pub struct RabbitMqPagination {
    pub page: u32,
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
    pub page: u32,
    pub page_size: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_regex: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RabbitMqPaginatedResponse<T> {
    pub filtered_count: u32,
    pub item_count: u32,
    pub items: Vec<T>,
    pub page: u32,
    pub page_count: u32,
    pub page_size: u32,
    pub total_count: u32,
}

impl Default for RabbitMqPagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: None,
            filter: None,
        }
    }
}

impl From<RabbitMqPagination> for RabbitMqPaginationRequest {
    fn from(value: RabbitMqPagination) -> Self {
        let (name, use_regex) = match value.filter {
            None => (None, None),
            Some(f) => match f {
                RabbitMqPaginationFilter::StringFilter(s) => (Some(s), Some(false)),
                RabbitMqPaginationFilter::RegexFilter(r) => (Some(r), Some(true)),
            },
        };

        Self {
            page: value.page,
            page_size: value.page_size.unwrap_or(PAGINATION_PAGE_SIZE),
            name,
            use_regex,
        }
    }
}
