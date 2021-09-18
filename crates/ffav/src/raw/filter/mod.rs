pub mod audio;
mod graph;
pub mod video;

use crate::{
    config::Dictionary,
    error::{Error, Result},
    Frame,
};
use ffav_sys::{avfilter_get_by_name, AVFilterContext};
use std::ffi::CString;

pub use graph::FilterGraph;

pub struct InputHandle<F>(pub(crate) FilterHandle<F>);
pub struct OutputHandle<F>(pub(crate) FilterHandle<F>);

pub struct AbstractHandle<'h> {
    graph_id: usize,
    filter_id: usize,
    filter_name: &'h String,
    filter_type_name: CString,
}

/// A handle to a filter owened by a
#[derive(Debug, Clone)]
pub struct FilterHandle<F> {
    graph_id: usize,
    filter_id: usize,
    config: F,
}

impl<I: FilterInput> FilterHandle<I> {
    pub fn as_input(self) -> InputHandle<I> {
        InputHandle(self)
    }
}

impl<O: FilterOutput> FilterHandle<O> {
    pub fn as_output(self) -> OutputHandle<O> {
        OutputHandle(self)
    }
}

/// This trait is implemented by structures which represent a filter object
pub trait Filter {
    /// The filter type structure from ffmpeg
    ///
    /// In the default implementation this will get the filter with `avfilter_get_by_name()`
    /// using the `Self::filter_type_name()` string.
    fn filter() -> crate::error::Result<*const ffav_sys::AVFilter> {
        unsafe {
            let filter_name = Self::filter_type_name();

            let out = avfilter_get_by_name(filter_name.as_ptr());

            if out.is_null() {
                return Err(Error::FilterNotFound(filter_name));
            }

            Ok(out)
        }
    }

    /// Get the name used to query ffmpeg for this filter type
    fn filter_type_name() -> CString;

    /// Get the name of the filter this must be unique for every filter in the FilterGraph
    fn filter_name(&self) -> &String;

    /// Get the dictionary of options to initially configure the filter
    fn config_parameters_dict(&self) -> Dictionary;
}

pub trait HasInputPads {}
pub trait HasOutputPads {}

/// Indicates a type which can be used as input to the FilterGraph
pub trait FilterInput {
    type StreamType;

    /// Submit a fram to the appropriate input type filter
    ///
    /// # Safety
    /// Thsi function is used internally by the FilterGraph and shouldn't be
    /// called directly by a user. The appropriate `*mut FilterContext` will be
    /// provided by the FilterGraph.
    unsafe fn submit_frame(
        filter: *mut AVFilterContext,
        frame: &mut Frame<Self::StreamType>,
    ) -> Result<()>;
}
/// Indicates a type which can be used as output from the FilterGraph
pub trait FilterOutput {
    type StreamType;

    /// Get a frame from the FilterGraph output and place it into an allocated frame
    ///
    /// # Safety
    /// Thsi function is used internally by the FilterGraph and shouldn't be
    /// called directly by a user. The appropriate `*mut FilterContext` will be
    /// provided by the FilterGraph.
    unsafe fn get_frame_into(
        filter: *mut AVFilterContext,
        frame: &mut Frame<Self::StreamType>,
    ) -> Result<()>;

    /// Get a frame from the FilterGraph output and return a newly allocated frame
    ///
    /// # Safety
    /// Thsi function is used internally by the FilterGraph and shouldn't be
    /// called directly by a user. The appropriate `*mut FilterContext` will be
    /// provided by the FilterGraph.
    unsafe fn get_frame(filter: *mut AVFilterContext) -> Result<Frame<Self::StreamType>> {
        let mut frame = Frame::new();

        Self::get_frame_into(filter, &mut frame)?;

        Ok(frame)
    }
}

pub trait Handle {
    type FilterType: Filter;

    fn graph_id(&self) -> usize;
    fn filter_id(&self) -> usize;
    fn filter_config(&self) -> &Self::FilterType;

    fn abstract_handle(&self) -> AbstractHandle<'_> {
        AbstractHandle {
            graph_id: self.graph_id(),
            filter_id: self.filter_id(),
            filter_name: self.filter_config().filter_name(),
            filter_type_name: Self::FilterType::filter_type_name(),
        }
    }
}

impl<F: Filter> Handle for FilterHandle<F> {
    type FilterType = F;

    fn graph_id(&self) -> usize {
        self.graph_id
    }

    fn filter_id(&self) -> usize {
        self.filter_id
    }

    fn filter_config(&self) -> &Self::FilterType {
        &self.config
    }
}

impl<F: Filter + FilterInput> Handle for InputHandle<F> {
    type FilterType = F;

    fn graph_id(&self) -> usize {
        self.0.graph_id
    }

    fn filter_id(&self) -> usize {
        self.0.filter_id
    }

    fn filter_config(&self) -> &Self::FilterType {
        &self.0.config
    }
}

impl<F: Filter + FilterOutput> Handle for OutputHandle<F> {
    type FilterType = F;

    fn graph_id(&self) -> usize {
        self.0.graph_id
    }

    fn filter_id(&self) -> usize {
        self.0.filter_id
    }

    fn filter_config(&self) -> &Self::FilterType {
        &self.0.config
    }
}

// TODO: Support updatable filter configurations
// pub trait UpdatableFilter: FilterParameters {...}
