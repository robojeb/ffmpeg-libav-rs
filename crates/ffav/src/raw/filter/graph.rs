use super::{
    AbstractHandle, Filter, FilterHandle, FilterInput, FilterOutput, Handle, HasInputPads,
    HasOutputPads, InputHandle, OutputHandle,
};
use crate::{
    error::{Error, Result},
    tags::{Configured, Unconfigured},
    util::dict::Dictionary,
    Frame,
};
use ffav_sys::{
    avfilter_graph_alloc, avfilter_graph_alloc_filter, avfilter_graph_config, avfilter_graph_free,
    avfilter_init_dict, avfilter_link, AVFilter, AVFilterContext, AVFilterGraph,
};
use std::{ffi::CString, marker::PhantomData};

pub struct FilterGraph<State> {
    graph: *mut AVFilterGraph,
    filters: Vec<*mut AVFilterContext>,
    _state: PhantomData<State>,
}

impl FilterGraph<Unconfigured> {
    pub fn new() -> Result<FilterGraph<Unconfigured>> {
        let graph = unsafe {
            let graph = avfilter_graph_alloc();

            if graph.is_null() {
                return Err(Error::AllocationFailed("creating filter graph"));
            }

            graph
        };

        Ok(FilterGraph {
            graph,
            filters: Vec::new(),
            _state: PhantomData,
        })
    }

    /// Add a filter that can act as an input to the FilterGraph
    ///
    /// This is shorthand for `add(config).as_input()` for types which are
    /// known to be `FilterInput`
    pub fn add_input<F>(&mut self, config: F) -> Result<InputHandle<F>>
    where
        F: Filter + FilterInput,
    {
        Ok(self.add(config)?.as_input())
    }

    /// Add a filter that can act as an output to the FilterGraph
    ///
    /// This is shorthand for `add(config).as_output()` for types which are
    /// known to be `FilterOutput`
    pub fn add_output<F>(&mut self, config: F) -> Result<OutputHandle<F>>
    where
        F: Filter + FilterOutput,
    {
        Ok(self.add(config)?.as_output())
    }

    /// Add and configure a new filter in this FilterGraph.
    ///
    /// This will return a handle to the Filter which can be used for further operations
    /// and contains the current configuration
    pub fn add<F>(&mut self, config: F) -> Result<FilterHandle<F>>
    where
        F: Filter,
    {
        let id = self.inner_add(
            config.filter_name(),
            F::filter()?,
            &mut config.config_parameters_dict(),
        )?;

        Ok(FilterHandle {
            graph_id: self.graph as usize,
            config,
            filter_id: id,
        })
    }

    /// Reduce code size of monomorphization by passing critical data to a shared
    /// inner function
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
            let cfilter_name =
                CString::new(name).map_err(|_| Error::InvalidFilterName(name.into()))?;

            let fctx = avfilter_graph_alloc_filter(self.graph, filter, cfilter_name.as_ptr());

            if fctx.is_null() {
                return Err(Error::AllocationFailed("creating filter"));
            }

            let err = avfilter_init_dict(fctx, config_dict.as_dict());
            if err < 0 {
                return Err(Error::from_av_err("setting filter parameters", err));
            }

            // Get the id of the new filter
            let id = self.filters.len();
            self.filters.push(fctx);

            Ok(id)
        }
    }

    /// Connect two filters `src` and `dst` from the Ouput indexed `src_pad` and the input indexed `dest_pad`
    pub fn connect<OH, F, IH, G>(
        &mut self,
        src: &OH,
        src_pad: u32,
        dest: &IH,
        dest_pad: u32,
    ) -> Result<()>
    where
        OH: Handle<FilterType = F>,
        F: Filter + HasOutputPads,
        IH: Handle<FilterType = G>,
        G: Filter + HasInputPads,
    {
        self.inner_connect(
            src.abstract_handle(),
            src_pad,
            dest.abstract_handle(),
            dest_pad,
        )
    }

    // Inner connect function reduces the size of the monomorphizations generated
    fn inner_connect<'h>(
        &mut self,
        src: AbstractHandle<'h>,
        src_pad: u32,
        dest: AbstractHandle<'h>,
        dest_pad: u32,
    ) -> Result<()> {
        // SAFETY: We aren't dereferencing the graph and we don't care if what it
        // points to is valid. We are merely using the pointer as an identifying
        // mark. Because a user could drop and then recreate a FilterGraph with
        // the same pointer htis can have a false Positive (eg filter which doesn't
        // belong to this graph still is considered as owned because of the pointer
        // match) but it isn't feasible to have the handle tied to the lifetime
        // because it complicates the borrow and Send/Sync details.
        if src.graph_id != self.graph as usize {
            return Err(Error::GraphDoesntOwnHandle);
        }

        if dest.graph_id != self.graph as usize {
            return Err(Error::GraphDoesntOwnHandle);
        }

        // TODO: Do we need to check that the name matches what we expect? or is that too much?
        let filter_src = self
            .filters
            .get(src.filter_id)
            // This should never happen because this handle should belong
            // to this graph, but we will check here just in case because
            // its possible someone deallocated and reallocated the FilterGraph
            // and got the same pointer so we passed the previous checks
            .ok_or(Error::FilterNotRegisteredWithGraph {
                name: src.filter_name.clone(),
            })?;

        let filter_dest =
            self.filters
                .get(dest.filter_id)
                .ok_or(Error::FilterNotRegisteredWithGraph {
                    name: dest.filter_name.clone(),
                })?;

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
                return Err(Error::OutputPadDoesntExist {
                    name: src.filter_name.clone(),
                    filter_type: src.filter_type_name,
                    pad_number: src_pad,
                });
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
                return Err(Error::InputPadDoesntExist {
                    name: dest.filter_name.clone(),
                    filter_type: dest.filter_type_name,
                    pad_number: dest_pad,
                });
            }

            // SAFETY: We know that both input pads exist and are not null from
            // the checks above, these should be safe to get

            // FIXME: This needs to be done on pads, not links
            // let output_pad = *(**filter_src).outputs.add(src_pad as usize);
            // let input_pad = *(**filter_dest).inputs.add(dest_pad as usize);

            // if (*output_pad).type_ != (*input_pad).type_ {
            //     return Err(Error::PadTypeMismatch {
            //         src_type: (*output_pad).type_,
            //         dest_type: (*input_pad).type_,
            //     });
            // }

            let err = avfilter_link(*filter_src, src_pad, *filter_dest, dest_pad);
            if err < 0 {
                // Theoretically this won't happen because we checked above, but just in case
                return Err(Error::from_av_err("linking filters", err));
            }
        }

        Ok(())
    }

    pub fn configure(self) -> Result<FilterGraph<Configured>> {
        // SAFETY: We know that `self.graph` is non-NULL because we have not been
        // dropped.
        unsafe {
            //TODO: At some point maybe we want to figure out the log_ctx to get more useful error messages
            let err = avfilter_graph_config(self.graph, std::ptr::null_mut());

            if err < 0 {
                return Err(Error::from_av_err("configuring filter graph", err));
            }
        }

        // SAFETY: The layout of both filter states is identical because
        // the marker `_state` is a ZST
        unsafe { Ok(std::mem::transmute(self)) }
    }
}

impl FilterGraph<Configured> {
    /// Submit input to the FilterGraph through the specified input
    pub fn submit_input<F: Filter + FilterInput>(
        &mut self,
        input: &InputHandle<F>,
        frame: &mut Frame<F::StreamType>,
    ) -> Result<()>
    where
        InputHandle<F>: Handle,
    {
        if input.graph_id() != self.graph as usize {
            return Err(Error::GraphDoesntOwnHandle);
        }

        let filter = self
            .filters
            .get(input.filter_id())
            // This should never happen because this handle should belong
            // to this graph, but we will check here just in case because
            // its possible someone deallocated and reallocated the FilterGraph
            // and got the same pointer so we passed the previous checks
            .ok_or(Error::FilterNotRegisteredWithGraph {
                name: input.filter_config().filter_name().clone(),
            })?;

        // SAFETY: We know that the filter is not NULL and that the filter type
        // matches the frame type being passed in
        unsafe {
            F::submit_frame(*filter, frame).map_err(|e| match e {
                // If the underlying implementation didn't catch an EGAIN we should
                // escalate it as the more useful `CouldNotAcceptInput`
                Error::AVUnknown {
                    ctx: _,
                    ret_val: -11,
                } => Error::CouldNotAcceptInput,
                _ => e,
            })
        }
    }

    /// Get output from the FilterGraph from the specified output and place it in the provided Frame
    ///
    /// Existing data in the frame will be unreferenced
    pub fn get_output_into<F: Filter + FilterOutput>(
        &mut self,
        output: &OutputHandle<F>,
        frame: &mut Frame<F::StreamType>,
    ) -> Result<()> {
        if output.graph_id() != self.graph as usize {
            return Err(Error::GraphDoesntOwnHandle);
        }

        let filter = self
            .filters
            .get(output.filter_id())
            // This should never happen because this handle should belong
            // to this graph, but we will check here just in case because
            // its possible someone deallocated and reallocated the FilterGraph
            // and got the same pointer so we passed the previous checks
            .ok_or(Error::FilterNotRegisteredWithGraph {
                name: output.filter_config().filter_name().clone(),
            })?;

        frame.unref();

        unsafe {
            F::get_frame_into(*filter, frame).map_err(|e| match e {
                // If the underlying implementation didn't catch an EGAIN we should
                // escalate it as the more useful `SubmitMoreInput`
                Error::AVUnknown {
                    ctx: _,
                    ret_val: -11,
                } => Error::SubmitMoreInput,
                _ => e,
            })
        }
    }

    /// Get output from the FilterGraph from the specified output
    pub fn get_output<F: Filter + FilterOutput>(
        &mut self,
        output: &OutputHandle<F>,
    ) -> Result<Frame<F::StreamType>> {
        if output.graph_id() != self.graph as usize {
            return Err(Error::GraphDoesntOwnHandle);
        }

        let filter = self
            .filters
            .get(output.filter_id())
            // This should never happen because this handle should belong
            // to this graph, but we will check here just in case because
            // its possible someone deallocated and reallocated the FilterGraph
            // and got the same pointer so we passed the previous checks
            .ok_or(Error::FilterNotRegisteredWithGraph {
                name: output.filter_config().filter_name().clone(),
            })?;

        unsafe {
            F::get_frame(*filter).map_err(|e| match e {
                // If the underlying implementation didn't catch an EGAIN we should
                // escalate it as the more useful `SubmitMoreInput`
                Error::AVUnknown {
                    ctx: _,
                    ret_val: -11,
                } => Error::SubmitMoreInput,
                _ => e,
            })
        }
    }
}

impl<T> std::ops::Drop for FilterGraph<T> {
    fn drop(&mut self) {
        // SAFETY: `avfilter_graph_free` will check if the graph provided is NULL
        // so even if this drop is called twice somehow, or the FilterGraph was
        // corrupted this will not cause a memory safety issue.
        unsafe {
            avfilter_graph_free(&mut self.graph);
            // NOTE: We don't have to free the filter's because the graph free
            // will take care of that for us
        }
    }
}
