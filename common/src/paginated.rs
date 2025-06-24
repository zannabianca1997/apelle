//! Paginated results

use std::ops::Deref;

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, PartialSchema, ToSchema, openapi};

/// A page of results
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Paginated<Item, Cursor = u32> {
    /// The items in the page
    pub items: Vec<Item>,
    /// Information about the page
    pub page_info: PageInfo<Cursor>,
}

impl<T> IntoIterator for Paginated<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T> Deref for Paginated<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

/// Information about a page
#[derive(Debug, Serialize, Deserialize, Clone, Copy, ToSchema)]
pub struct PageInfo<Cursor = u32> {
    /// The size of the page
    pub size: u32,
    /// The total number of items
    pub total: Option<u32>,

    /// The cursor to the first page, if any
    pub first: Option<Cursor>,
    /// The cursor to the previous page, if any
    pub prev: Option<Cursor>,
    /// The cursor to the current page
    pub page: Cursor,
    /// The cursor to the next page, if any
    pub next: Option<Cursor>,
    /// The cursor to the last page, if any
    pub last: Option<Cursor>,
}

/// Pagination parameters
#[derive(Debug, Serialize, Deserialize, Clone, Copy, ToSchema)]
pub struct PaginationParams<Cursor = u32> {
    #[serde(default = "default_page_size")]
    pub page_size: u32,
    pub page: Option<Cursor>,
}

impl<Cursor> IntoParams for PaginationParams<Cursor>
where
    Option<Cursor>: PartialSchema,
{
    fn into_params(
        parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
    ) -> Vec<utoipa::openapi::path::Parameter> {
        Vec::from([
            openapi::path::Parameter::builder()
                .name("page_size")
                .schema(Some(u32::schema()))
                .description(Some("Maximum number of items per page"))
                .parameter_in(
                    parameter_in_provider().unwrap_or(utoipa::openapi::path::ParameterIn::Query),
                )
                .build(),
            openapi::path::Parameter::builder()
                .name("page")
                .schema(Some(Option::<Cursor>::schema()))
                .description(Some("Requested page"))
                .parameter_in(
                    parameter_in_provider().unwrap_or(utoipa::openapi::path::ParameterIn::Query),
                )
                .build(),
        ])
    }
}

fn default_page_size() -> u32 {
    10
}
