use super::{AbstractFilterInfo, FilterHandle};
use crate::{
    config::Dictionary,
    error::{Error, Result},
    raw::Frame,
    util::{
        make_id_from_ptr,
        marker::{Configured, Unconfigured},
        traits::{Filter, GraphInput, GraphOutput, HasInputPads, HasOutputPads},
    },
};
use ffav_sys::{
    avfilter_graph_alloc, avfilter_graph_alloc_filter, avfilter_init_dict, avfilter_link, AVFilter,
    AVFilterContext, AVFilterGraph,
};
use std::{ffi::CString, marker::PhantomData};

/// A Graph of filters which process Audio and Video streams
pub struct FilterGraph<Status> {
    graph: *mut AVFilterGraph,
    filters: Vec<*mut AVFilterContext>,
    _status: PhantomData<Status>,
}

impl FilterGraph<Unconfigured> {
    /// Create a new empty filter graph
    ///
    /// # Panics
    /// If memory cannot be allocated for the filter graph
    pub fn new() -> Result<FilterGraph<Unconfigured>> {
        unsafe {
            let graph = avfilter_graph_alloc();

            if graph.is_null() {
                panic!("Could not allocate a new filter graph");
            }

            Ok(FilterGraph {
                graph,
                filters: Vec::new(),
                _status: PhantomData,
            })
        }
    }

    /// Add a filter to the filter graph and get back a `FilterHandle`
    pub fn add_filter<F: Filter>(&mut self, filter_config: F) -> Result<FilterHandle<F>> {
        let id = self.inner_add(
            filter_config.filter_name(),
            F::get_filter()?,
            &mut filter_config.get_parameter_dictionary(),
        )?;

        Ok(FilterHandle {
            graph: make_id_from_ptr(self.graph),
            config: filter_config,
            index: id,
        })
    }

    // Helper function to reduce the amount of LLVM-IR submitted to the back-end
    // when a large number of filters is instantiated
    fn inner_add(
        &mut self,
        name: &str,
        filter: *const AVFilter,
        config_dict: &mut Dictionary,
    ) -> Result<usize> {
        // SAFETY: This code relies on the functions `avfilter_graph_alloc_filter()`
        // and `avfilter_init_dict()` to not violate memory safety.
        // From our code's end, we check that the returned AVFilterContext pointer
        // is not NULL before proceeding with configuration and placing it into
        // the filters vector
        unsafe {
            let cfilter_name = CString::new(name)?;

            let fctx = avfilter_graph_alloc_filter(self.graph, filter, cfilter_name.as_ptr());

            if fctx.is_null() {
                panic!("Could not allocate memory for FilterContext");
            }

            let err = avfilter_init_dict(fctx, config_dict.as_dict());
            if err < 0 {
                return Err(Error::Unknown("Error while initializing filter"));
                //return Err(Error::from_av_err("setting filter parameters", err));
            }

            // Get the id of the new filter
            let id = self.filters.len();
            self.filters.push(fctx);

            Ok(id)
        }
    }

    /// Link the output pad from `src` to the input pad of `dst`
    pub fn link<A: HasOutputPads, B: HasInputPads>(
        &mut self,
        src: FilterHandle<A>,
        src_pad: u32,
        dest: FilterHandle<B>,
        dest_pad: u32,
    ) -> Result<()> {
        self.inner_link(
            src.to_filter_info(),
            src_pad,
            dest.to_filter_info(),
            dest_pad,
        )
    }

    // Helper function to reduce the amount of LLVM-IR passed to the back end
    // by all the instantiations of `link`
    fn inner_link(
        &mut self,
        src: AbstractFilterInfo,
        src_pad: u32,
        dest: AbstractFilterInfo,
        dest_pad: u32,
    ) -> Result<()> {
        let graph_id = make_id_from_ptr(self.graph);
        if src.graph != graph_id {
            return Err(Error::Unknown(
                "The filter provided didn't belong to the FilterGraph",
            ));
        }
        if dest.graph != graph_id {
            return Err(Error::Unknown(
                "The filter provided didn't belong to the FilterGraph",
            ));
        }

        // TODO: Do we need to check that the name matches what we expect? or is that too much?
        let filter_src = self
            .filters
            .get(src.index)
            // This should never happen because this handle should belong
            // to this graph, but we will check here just in case because
            // its possible someone deallocated and reallocated the FilterGraph
            // and got the same pointer so we passed the previous checks
            .ok_or(Error::Unknown(
                "Could not find the requestd filter by index",
            ))?;

        let filter_dest = self.filters.get(dest.index).ok_or(Error::Unknown(
            "Could not find the requestd filter by index",
        ))?;

        // SAFETY: We know that filter_a and filter_b are non-NULL because we
        // checked the pointers before putting them into the filters vector
        // the only way to add a filter is through `FillterGraph::add`.
        // Also the filters contain internal back links to the `AVFilterGraph`
        // that we own, which we know cannot have been deallocated.
        unsafe {
            // NOTE: Here we check that the pads which the user requested
            //     a) Exist
            //     b) Are of compatible types
            // Technically `avfilter_link()` will do these checks too, but it
            // will return just `EINVAL` which isn't very helpful.
            // Because this is a configuration step I justify that it is okay to
            // double check the types match and that the pad count's match so
            // that we can return a useful error message.

            // SAFETY: It is safe to index the array `outputs` by `src_pad`
            // because we know that there are at least as many outputs per
            // `nb_outputs`, if this isn't true ffmpeg has violated an invariant
            // that we cannot check
            let output_slice = std::slice::from_raw_parts(
                (**filter_src).outputs,
                (**filter_src).nb_outputs as usize,
            );
            if output_slice.len() <= src_pad as usize || !output_slice[src_pad as usize].is_null() {
                return Err(Error::Unknown("The requested output pad didn't exist"));
                // return Err(Error::OutputPadDoesntExist {
                //     name: src.filter_name.clone(),
                //     filter_type: src.filter_type_name,
                //     pad_number: src_pad,
                // });
            }

            // SAFETY: It is safe to index the array `inputs` by `dest_pad`
            // because we know that there are at least as many outputs per
            // `nb_inputs`, if this isn't true ffmpeg has violated an invariant
            // that we cannot check
            let input_slice = std::slice::from_raw_parts(
                (**filter_dest).inputs,
                (**filter_dest).nb_inputs as usize,
            );
            if input_slice.len() <= dest_pad as usize || !input_slice[dest_pad as usize].is_null() {
                return Err(Error::Unknown("The requested input pad didn't exist"));
                // return Err(Error::InputPadDoesntExist {
                //     name: dest.filter_name.clone(),
                //     filter_type: dest.filter_type_name,
                //     pad_number: dest_pad,
                // });
            }

            // TODO: There might be a way to check if the pad types match which
            // would make a nice addition to the errors we can provide

            let err = avfilter_link(*filter_src, src_pad, *filter_dest, dest_pad);
            if err < 0 {
                // Theoretically this won't happen because we checked above, but just in case
                return Err(Error::Unknown("Error while linking filters together"));
            }
        }

        Ok(())
    }
}

impl FilterGraph<Configured> {
    /// Submit a frame to the FilterGraph at the specified node
    pub fn submit_frame<F: GraphInput>(
        &mut self,
        filter: FilterHandle<F>,
        frame: &mut Frame<F::StreamType>,
    ) -> Result<()> {
        let input_node = self
            .filters
            .get(filter.index)
            // This should never happen because this handle should belong
            // to this graph, but we will check here just in case because
            // its possible someone deallocated and reallocated the FilterGraph
            // and got the same pointer so we passed the previous checks
            .ok_or(Error::Unknown(
                "Could not find the requestd filter by index",
            ))?;

        unsafe { F::submit_frame(*input_node, frame) }
    }

    /// Get a frame from the FilterGraph at the specified node
    pub fn get_frame_into<F: GraphOutput>(
        &mut self,
        filter: FilterHandle<F>,
        frame: &mut Frame<F::StreamType>,
    ) -> Result<()> {
        let output_node = self
            .filters
            .get(filter.index)
            // This should never happen because this handle should belong
            // to this graph, but we will check here just in case because
            // its possible someone deallocated and reallocated the FilterGraph
            // and got the same pointer so we passed the previous checks
            .ok_or(Error::Unknown(
                "Could not find the requestd filter by index",
            ))?;

        unsafe { F::get_frame_into(*output_node, frame) }
    }

    /// Get a frame from the FilterGraph at the specified node and return a new Frame object with that data
    ///
    /// # Panics
    /// Will panic if memory for the new frame cannot be allocated
    pub fn get_frame<F: GraphOutput>(
        &mut self,
        filter: FilterHandle<F>,
    ) -> Result<Frame<F::StreamType>> {
        let mut frame = Frame::new();

        self.get_frame_into(filter, &mut frame)?;

        Ok(frame)
    }
}
