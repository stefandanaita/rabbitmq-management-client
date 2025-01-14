pub mod pagination;
pub mod sorting;

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct RabbitMqRequestOptions {
    pub disable_stats: bool,
    pub pagination: Option<pagination::RabbitMqPagination>,
    pub sorting: Option<sorting::RabbitMqSorting>,
}

