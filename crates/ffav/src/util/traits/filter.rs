use std::ffi::CString;

use ffav_sys::{avfilter_get_by_name, AVFilter, AVFilterContext};

use crate::{
    config::Dictionary,
    error::{Error, Result},
    raw::Frame,
};

use super::MediaMarker;

/// Indicates a type of Filter
///
/// The structure contains the configuration information for the Filter
pub trait Filter {
    /// Get the CString name of the filter type to fetch
    fn get_filter_type_name() -> CString;

    /// Get the AVFilter instance for this filter type
    fn get_filter() -> Result<*const AVFilter> {
        unsafe {
            let filter_name = Self::get_filter_type_name();

            let out = avfilter_get_by_name(filter_name.as_ptr());

            if out.is_null() {
                return Err(Error::FilterNotFound(filter_name));
            }

            Ok(out)
        }
    }

    /// Get the name of this filter should have in the Filter graph
    fn filter_name(&self) -> &str;

    /// Get the parameter dictionary to configure this filter
    fn get_parameter_dictionary(&self) -> Dictionary;
}

/// Indicates a filter which has parameters that can be updated at runtime
pub trait UpdatableFilter {
    /// Get the dictionary which contains the command parameters
    fn get_command_dictionary(&self) -> Dictionary;
}

/// Indicates that a Filter can accpet input from another Filter
pub trait HasInputPads: Filter {}

/// Indicates that a Filter produces output that can be passed to another Filter
pub trait HasOutputPads: Filter {}

/// Indicates that a Filter is an input point for the FilterGraph
pub trait GraphInput: Filter {
    /// The type of Frame that the FilterGraph expects to be submitted to this node
    type StreamType: MediaMarker;

    /// Handle submitting to this type of filter
    ///
    /// # Safety
    /// The `AVFilterContext` must be valid, part of a fully configured FilterGraph
    /// and must be of the type expected by the structure that implements this trait
    unsafe fn submit_frame(
        filter: *mut AVFilterContext,
        frame: &mut Frame<Self::StreamType>,
    ) -> Result<()>;
}

/// Indicates that a Filter is an output point from the FilterGraph
pub trait GraphOutput: Filter {
    /// The type of frame that the FilterGraph will produce from this node
    type StreamType: MediaMarker;

    /// Handle submitting to this type of filter
    ///
    /// # Safety
    /// The `AVFilterContext` must be valid, part of a fully configured FilterGraph
    /// and must be of the type expected by the structure that implements this trait
    unsafe fn get_frame_into(
        filter: *mut AVFilterContext,
        frame: &mut Frame<Self::StreamType>,
    ) -> Result<()>;
}
