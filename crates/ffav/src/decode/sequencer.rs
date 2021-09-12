use std::path::Path;

use crate::{
    error::Result,
    raw::{
        codec::Codec,
        filter::{FilterGraph, InputHandle},
        format::Format,
        stream::Stream,
    },
    tags::{Configured, Decode, Input, Unconfigured},
    util::{Filterable, MediaType},
};

pub struct SimpleSequencer<AV: Filterable, State> {
    items: Vec<Item<AV>>,
    filter_graph: FilterGraph<State>,
}

impl<AV: Filterable + MediaType> SimpleSequencer<AV, Unconfigured> {
    pub fn new() -> Result<Self> {
        Ok(SimpleSequencer {
            items: Vec::new(),
            filter_graph: FilterGraph::new()?,
        })
    }

    pub fn add_item<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let input_format = Format::open_input(path)?;

        let best_stream: Stream<AV> = input_format.get_best_stream().unwrap();

        let codec = Codec::open_decode(&best_stream)?;

        let src = AV::from_decoded_stream(
            format!("src_{}", self.items.len()),
            codec.out_stream_config(),
        );

        let src = self.filter_graph.add_input(src)?;

        let item = Item {
            input: input_format,
            codec,
            buf: src,
        };

        self.items.push(item);

        Ok(())
    }

    pub fn configure(self) -> SimpleSequencer<AV, Configured> {
        unimplemented!()
    }
}

struct Item<AV: Filterable> {
    input: Format<Input>,
    codec: Codec<Decode, AV>,
    buf: InputHandle<AV::InputType>,
}
