//! Paginated results

use std::ops::Deref;

use serde::{Deserialize, Serialize};

/// A page of results
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct PageInfo<Cursor = u32> {
    /// The size of the page
    pub size: u32,
    /// The total number of items
    pub total: Option<u32>,
    /// The number of the page
    pub page: u32,
    /// The cursor to the first page, if any
    pub first: Option<Cursor>,
    /// The cursor to the previous page, if any
    pub prev: Option<Cursor>,
    /// The cursor to the next page, if any
    pub next: Option<Cursor>,
    /// The cursor to the last page, if any
    pub last: Option<Cursor>,
}

impl PageInfo<u32> {
    /// Create a page info for a pagination with all pages of the same size
    pub fn regular(
        page: u32,
        total: Option<u32>,
        size: u32,
        page_size: u32,
        next_present: bool,
    ) -> Self {
        let last = total.map(|total| total.saturating_sub(1) / page_size);
        let next = next_present.then_some(page + 1);
        let prev = page.checked_sub(1);
        Self {
            size,
            total,
            page,
            first: Some(0),
            prev,
            next,
            last,
        }
    }
}

/// Pagination parameters
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct PaginationParams<Cursor = u32> {
    #[serde(default = "default_page_size")]
    pub page_size: u32,
    pub page: Option<Cursor>,
}

fn default_page_size() -> u32 {
    10
}
