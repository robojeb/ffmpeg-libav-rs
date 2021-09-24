//! Structures for defining Filters and a FilterGraph
mod graph;

pub use graph::*;

use crate::util::traits::Filter;

/// A Handle into a FilterGraph for a given Filter
pub struct FilterHandle<F: Filter> {
    graph: u64,
    index: usize,
    config: F,
}

impl<F: Filter> FilterHandle<F> {
    fn to_filter_info(&self) -> AbstractFilterInfo {
        AbstractFilterInfo {
            graph: self.graph,
            index: self.index,
        }
    }
}

impl<F: Filter> std::ops::Deref for FilterHandle<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

struct AbstractFilterInfo {
    pub graph: u64,
    pub index: usize,
}
